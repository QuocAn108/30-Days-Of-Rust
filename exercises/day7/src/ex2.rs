use crate::my_enum::{self, FileFormat};

pub fn print_ex2() {
    let pdf_file = FileFormat::PDF(100);
    my_enum::print_format_size(pdf_file);
    let word_file = FileFormat::Word(200);
    my_enum::print_format_size(word_file);
    let excel_file = FileFormat::Excel(300);
    my_enum::print_format_size(excel_file);

    let active_status = my_enum::Status::Active;
    my_enum::check_status(active_status);
    let inactive_status = my_enum::Status::Inactive;
    my_enum::check_status(inactive_status);
    let suspended_status = my_enum::Status::Suspended;
    my_enum::check_status(suspended_status);

    let pending_status = my_enum::OrderStatus::Pending;
    my_enum::check_order_status(pending_status);
    let shipped_status = my_enum::OrderStatus::Shipped;
    my_enum::check_order_status(shipped_status);
    let delivered_status = my_enum::OrderStatus::Delivered;
    my_enum::check_order_status(delivered_status);

    let circle = my_enum::Shape::Circle(5.0);
    my_enum::calculate_area(circle);
    let rectangle = my_enum::Shape::Rectangle(4.0, 6.0);
    my_enum::calculate_area(rectangle);
    let triangle = my_enum::Shape::Triangle(3.0, 4.0);
    my_enum::calculate_area(triangle);
}
