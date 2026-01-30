use diesel::prelude::*;
use crate::schema::{orders, order_details, payment_orders, products, stock_quants};
use crate::models::transaction::{Order, NewOrder, NewOrderDetail, NewPaymentOrder, CreateOrderRequest};
use crate::models::master::{Product, StockQuant};
use crate::db::DbPooledConn as DbConn;

#[derive(Debug)]
pub enum OrderError {
    InsufficientStock(String),
    ProductNotFound(i32),
    DatabaseError(diesel::result::Error),
}

impl std::fmt::Display for OrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderError::InsufficientStock(msg) => write!(f, "Insufficient stock: {}", msg),
            OrderError::ProductNotFound(id) => write!(f, "Product not found: {}", id),
            OrderError::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}

impl std::error::Error for OrderError {}

impl From<diesel::result::Error> for OrderError {
    fn from(err: diesel::result::Error) -> Self {
        OrderError::DatabaseError(err)
    }
}

pub struct OrderRepository;

impl OrderRepository {
    pub fn list(conn: &mut DbConn) -> QueryResult<Vec<Order>> {
        orders::table.select(Order::as_select()).load(conn)
    }

    pub fn find_by_id(conn: &mut DbConn, id: i32) -> QueryResult<Order> {
        orders::table.find(id).first(conn)
    }

    pub fn create_transactional(conn: &mut DbConn, req: CreateOrderRequest) -> Result<Order, OrderError> {
        conn.transaction(|conn| {
            let mut amount_total = 0.0;
            
            for item in &req.items {
                // 1. Fetch product name for error reporting
                let product: Product = products::table.find(item.product_id)
                    .first(conn)
                    .map_err(|_| OrderError::ProductNotFound(item.product_id))?;

                // 2. Fetch and LOCK the stock quant record for the specific location
                // Using .for_update() ensures no other transaction can modify this stock until we are done.
                let quant: Option<StockQuant> = stock_quants::table
                    .filter(stock_quants::product_id.eq(item.product_id))
                    .filter(stock_quants::location_id.eq(req.location_id))
                    .for_update()
                    .first(conn)
                    .optional()?;

                let available_stock = quant.map(|q| q.quantity).unwrap_or(0.0);

                if available_stock < item.quantity {
                    return Err(OrderError::InsufficientStock(format!(
                        "{} (Requested: {}, Available: {})", 
                        product.name, item.quantity, available_stock
                    )));
                }

                // 3. Update stock in the specific location
                diesel::update(stock_quants::table)
                    .filter(stock_quants::product_id.eq(item.product_id))
                    .filter(stock_quants::location_id.eq(req.location_id))
                    .set(stock_quants::quantity.eq(stock_quants::quantity - item.quantity))
                    .execute(conn)?;

                amount_total += item.quantity * item.price_unit;
            }

            // 4. Insert Order record
            let new_order = NewOrder {
                name: req.name.clone(),
                customer_id: req.customer_id,
                amount_total,
            };

            let order = diesel::insert_into(orders::table)
                .values(&new_order)
                .returning(Order::as_returning())
                .get_result(conn)?;

            // 5. Insert OrderDetails
            for item in req.items {
                let new_detail = NewOrderDetail {
                    order_id: order.id,
                    product_id: item.product_id,
                    quantity: item.quantity,
                    price_unit: item.price_unit,
                    amount_total: item.quantity * item.price_unit,
                };

                diesel::insert_into(order_details::table)
                    .values(&new_detail)
                    .execute(conn)?;
            }

            // 6. Insert PaymentOrders
            for payment in req.payments {
                let new_payment = NewPaymentOrder {
                    order_id: order.id,
                    payment_method_id: payment.payment_method_id,
                    amount_paid: payment.amount_paid,
                };

                diesel::insert_into(payment_orders::table)
                    .values(&new_payment)
                    .execute(conn)?;
            }

            Ok(order)
        })
    }

    pub fn delete(conn: &mut DbConn, id: i32) -> QueryResult<usize> {
        diesel::delete(orders::table.find(id)).execute(conn)
    }
}
