use rocket::form::Form;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

use crate::auth::guards::User;
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
pub fn dashboard(user: User) -> Template {
    Template::render(
        "pages/dashboard/index",
        context! {
            title: "Dashboard",
            user_name: user.name,
        },
    )
}

#[get("/dashboard")]
pub fn dashboard_redirect() -> Redirect {
    Redirect::to("/login")
}

#[get("/dashboard/orders")]
pub fn view_orders(user: User) -> Template {
    Template::render(
        "pages/dashboard/orders",
        context! {
            title: "Your Orders",
            user_name: user.name,
        },
    )
}

#[get("/dashboard/orders")]
pub fn view_orders_redirect() -> Redirect {
    Redirect::to("/login")
}

#[get("/dashboard/new-order")]
pub fn new_order_form(user: User) -> Template {
    Template::render(
        "pages/dashboard/new_order",
        context! {
            title: "Submit New Order",
            user_name: user.name,
        },
    )
}

#[get("/dashboard/new-order")]
pub fn new_order_form_redirect() -> Redirect {
    Redirect::to("/login")
}

#[post("/dashboard/new-order", data = "<form>")]
pub async fn process_new_order(_user: User, form: Form<OrderForm>, conn: DbConn) -> Redirect {
    let order_data = form.into_inner();

    let new_order = NewOrder {
        website: order_data.website,
        details: order_data.details,
        deadline: order_data.deadline,
    };

    let _ = conn
        .run(move |c| OrderService::create_order(&new_order, c))
        .await;

    Redirect::to("/dashboard/orders")
}
