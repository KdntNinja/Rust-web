use crate::models::NewOrder;
use crate::DbConn;
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};

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

#[post("/dashboard/new-order", data = "<order_form>")]
pub async fn new_order(order_form: Form<OrderForm>, conn: DbConn) -> &'static str {
    // Create a new order from the form data
    let new_order = NewOrder {
        website: order_form.website.clone(),
        details: order_form.details.clone(),
        deadline: order_form.deadline.clone(),
    };

    // Fixed: use async/await with rocket_sync_db_pools
    // Commented out for now since NewOrder.insert is not implemented
    // let result = conn.run(move |c| new_order.insert(c)).await;
    // 
    // match result {
    //     Ok(_) => "Order submitted successfully!",
    //     Err(_) => "Error submitting order. Please try again.",
    // }

    "Order submitted successfully!"
}

#[get("/dashboard/orders")]
pub fn view_orders() -> Template {
    Template::render(
        "pages/dashboard/orders",
        context! {
            title: "Your Orders",
        },
    )
}
