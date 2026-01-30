use diesel::prelude::*;
use crate::schema::{uoms, warehouses, locations, stock_quants};
use crate::models::master::{
    Uom, NewUom, 
    Warehouse, NewWarehouse, 
    Location, NewLocation,
    StockQuant, NewStockQuant
};
use crate::db::DbPooledConn as DbConn;

pub struct InventoryRepository;

impl InventoryRepository {
    /* UoM Logic */
    pub fn list_uoms(conn: &mut DbConn) -> QueryResult<Vec<Uom>> {
        uoms::table.select(Uom::as_select()).load(conn)
    }

    pub fn create_uom(conn: &mut DbConn, new_uom: NewUom) -> QueryResult<Uom> {
        diesel::insert_into(uoms::table)
            .values(&new_uom)
            .returning(Uom::as_returning())
            .get_result(conn)
    }

    /* Warehouse Logic */
    pub fn list_warehouses(conn: &mut DbConn) -> QueryResult<Vec<Warehouse>> {
        warehouses::table.select(Warehouse::as_select()).load(conn)
    }

    pub fn create_warehouse(conn: &mut DbConn, new_wh: NewWarehouse) -> QueryResult<Warehouse> {
        diesel::insert_into(warehouses::table)
            .values(&new_wh)
            .returning(Warehouse::as_returning())
            .get_result(conn)
    }

    /* Location Logic */
    pub fn list_locations(conn: &mut DbConn) -> QueryResult<Vec<Location>> {
        locations::table.select(Location::as_select()).load(conn)
    }

    pub fn create_location(conn: &mut DbConn, new_loc: NewLocation) -> QueryResult<Location> {
        diesel::insert_into(locations::table)
            .values(&new_loc)
            .returning(Location::as_returning())
            .get_result(conn)
    }

    /* Stock Quant Logic */
    pub fn list_stock(conn: &mut DbConn) -> QueryResult<Vec<StockQuant>> {
        stock_quants::table.select(StockQuant::as_select()).load(conn)
    }

    pub fn update_stock(conn: &mut DbConn, new_quant: NewStockQuant) -> QueryResult<StockQuant> {
        diesel::insert_into(stock_quants::table)
            .values(&new_quant)
            .on_conflict((stock_quants::product_id, stock_quants::location_id))
            .do_update()
            .set(stock_quants::quantity.eq(new_quant.quantity))
            .returning(StockQuant::as_returning())
            .get_result(conn)
    }
}
