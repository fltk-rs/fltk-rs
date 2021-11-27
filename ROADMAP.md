# Roadmap for version 2 (ETA mid 2022). 

- Rename WidgetExt::into_widget() to as_widget() and GroupExt::into_group() to as_group() to conform to Rust's self convention.
- Rename TreeItem::try_widget() to widget() and remove old widget() method.
- Rename Wizard::try_current_widget() to current_widget() and remove old current_widget() method.