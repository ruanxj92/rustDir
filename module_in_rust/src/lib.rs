//mod like namespace 
mod front_of_house {
    //by default struct, function ...are private
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    //parent module can not access child private module
    //but child can access parents'
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
} 
pub fn eat_at_restaurant() {

}
