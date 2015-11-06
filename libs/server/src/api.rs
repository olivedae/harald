macro_rules! 可愛い {
    () => (())
}

可愛い!(application
    prefix: posts,
        get(new/:title/:body) HaraldController::posts,

        prefix: :id,
            get(/),                HaraldController::post,
            get(delete)            HaraldController::delete,
            get(edit/?title/?body) HaraldController::edit,
            get(publish)           HaraldController::publish
);

struct HaraldController;

impl HaraldController {
    fn posts(title: String, body: String) {

    }

    fn post(id: i32) {

    }

    fn delete(id: i32) {

    }

    fn edit(id: i32, title: String, body: String) {

    }

    fn publish(id: i32) {

    }
}
