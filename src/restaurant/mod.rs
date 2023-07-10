mod pizza_order {
	// pub -> public, so that they can be accessed outside this module, in other files
	pub struct Pizza {
		pub dough: String,
		pub cheese: String,
		pub topping: String
	}
	impl Pizza {
		pub fn lunch(topping: &str) -> Pizza {
			Pizza {
				dough: String::from("Maize Regular"),
				cheese: String::from("Mozzarella"),
				topping: String::from(topping)
			}
		}
	}
	pub mod help_customer {

		fn seat_customer() {
			println!("Customer seated at a table");
		}
		//if mod is public (pub), children dont become pub automatic, to
		// access children outside mod, declare pub for children also
		pub fn take_order() {
			seat_customer();
			//super to reference the outer module from inner module
			let cus_pizza:super::Pizza = super::Pizza::lunch("Chicken kheema");
			serve_customer(cus_pizza);
		}

		fn serve_customer(cus_pizza: super::Pizza) {
			println!("Customer served pizza with {} toppings",cus_pizza.topping);
		}
	}
}

pub fn order_food() {
	crate::restaurant::pizza_order::help_customer::take_order();
}