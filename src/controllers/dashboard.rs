use rocket::form::Form;
use rocket_dyn_templates::{context, Template};

use crate::models::order::NewOrder;
use crate::services::order::OrderService;
use crate::DbConn;

#[derive(FromForm)]
pub struct OrderForm {
    website: String,
    details: String,
    deadline: String,
}

#[get("/dashboard")]
pub fn dashboard() -> Template {
    Template::render(
        "pages/dashboard/index",
        context! {
            title: "Dashboard",
        },
    )
}

#[get("/dashboard/orders")]
pub fn view_orders() -> Template {
    Template::render(
        "pages/orders",
        context! {
            title: "Your Orders",
        },
    )
}

#[get("/dashboard/new-order")]
pub fn new_order_form() -> Template {
    Template::render(
        "pages/dashboard/new_order",
        context! {
            title: "Submit New Order",
        },
    )
}
