fn main() {
    (|| {})(|| {
        //~^ ERROR this function takes 0 arguments but 1 argument was supplied
        let b = 1;
    });
}
