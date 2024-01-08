fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window {
        min_width: 400px;
        min_height: 400px;
        Text {
            text: "hello world";
            color: green;
        }
    }
}

