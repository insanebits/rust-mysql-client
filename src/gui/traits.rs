///
/// GUI Panel trait, such panels would be editor, sidebar, result list.
///

pub trait Panel
{
    fn get_widget(&self);
}
