pub mod instrument {
    pub fn clarinet() {
        println!("clarinet is called!");
        super::breath_in();
    }
}

fn breath_in() {
    println!("breath is called!");
}
