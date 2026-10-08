use crate::my_enum::{self, FileFormat};

pub fn print_ex2() {
    let pdf_file = FileFormat::PDF(100);
    my_enum::print_format_size(pdf_file);
    let word_file = FileFormat::Word(200);
    my_enum::print_format_size(word_file);
    let excel_file = FileFormat::Excel(300);
    my_enum::print_format_size(excel_file);

    let active_status = my_enum::OrderStatus::Active;
    my_enum::check_order_status(active_status);
    let inactive_status = my_enum::OrderStatus::Inactive;
    my_enum::check_order_status(inactive_status);
    let suspended_status = my_enum::OrderStatus::Suspended;
    my_enum::check_order_status(suspended_status);
}
