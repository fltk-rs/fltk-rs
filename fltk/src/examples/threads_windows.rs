//!How to show a window from another thread.
//!
//!The code can be found in the `examples` directory next to the `src` directory
//!in the source distribution.
//!
//!### Description
//!A standard counter containing a button for increment and decrement each. The
//!callback for each button spawns a thread for a long-running task. During the
//!task, we deactivate the buttons to avoid confusing UI behavior. After the task
//!is done, we show a message window to the user.
//!
//!A window can only be shown from the main thread, so we use a
//![`channel`](crate::app::channel) to send a message that makes the main thread
//!show the notification window. We also use messages to increment/decrement the
//!counter and activate/deactivate the buttons. Those things can be done without
//!messages, too, but the example keeps a consistent approach.
//!
//!The notification window itself contains a [`HelpView`](crate::misc::HelpView) to
//!show the message. It takes HTML syntax, which we make no use of.
