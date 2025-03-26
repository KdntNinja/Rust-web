use rocket::form::Form;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

use crate::auth::guards::User;
use crate::models::order::{NewOrder, mock as order_mock};

// Mark fields with #[allow(dead_code)] since they'll be used in form processing
#[derive(FromForm, Debug)]
pub struct OrderForm {
    #[allow(dead_code)]
    website: String,
    #[allow(dead_code)]
    details: String,
    #[allow(dead_code)]
    deadline: String,
}

#[get("/dashboard", rank = 1)]
pub fn dashboard(user: User) -> Template {
    Template::render(
        "pages/dashboard/index",
        context! {
            title: "Dashboard",
            user_name: user.username,
        },
    )
}

#[get("/dashboard", rank = 2)]
pub fn dashboard_redirect() -> Redirect {
    Redirect::to("/login")
}

#[get("/dashboard/orders", rank = 1)]
pub fn view_orders(user: User) -> Template {
    // Get orders from mock database
    let orders = order_mock::find_all();
    
    Template::render(
        "pages/dashboard/orders",
        context! {
            title: "Your Orders",
            user_name: user.username,
            orders: orders
        },
    )
}

#[get("/dashboard/orders", rank = 2)]
pub fn view_orders_redirect() -> Redirect {
    Redirect::to("/login")
}

#[get("/dashboard/new-order", rank = 1)]
pub fn new_order_form(user: User) -> Template {
    Template::render(
        "pages/dashboard/new_order",
        context! {
            title: "Submit New Order",
            user_name: user.username,
        },
    )
}

#[get("/dashboard/new-order", rank = 2)]
pub fn new_order_form_redirect() -> Redirect {
    Redirect::to("/login")
}

#[post("/dashboard/new-order", data = "<form>")]
pub fn process_new_order(_user: User, form: Form<OrderForm>) -> Redirect {
    let form_data = form.into_inner();
    println!("Processing new order: {:?}", form_data);
    
    // Create new order in mock database
    let new_order = NewOrder {
        website: form_data.website,
        details: form_data.details,
        deadline: form_data.deadline,
    };
    
    order_mock::insert(new_order);
    
    Redirect::to("/dashboard/orders")
}
