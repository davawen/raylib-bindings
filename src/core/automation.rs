use std::ffi::{CStr, CString};

use crate::{Raylib, ffi::{self, AutomationEvent}};

pub struct AutomationEventList(ffi::AutomationEventList);

impl Drop for AutomationEventList {
    fn drop(&mut self) {
        unsafe { ffi::UnloadAutomationEventList(self.0) }
    }
}

impl std::ops::Deref for AutomationEventList {
    type Target = [AutomationEvent];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.0.events, self.0.count as usize) }
    }
}

impl std::ops::DerefMut for AutomationEventList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.0.events, self.0.count as usize) }
    }
}

// TODO: Avoid automation event sigsegv??
// impl Raylib {
//     /// Load automation events list from file
//     /// Other methods related to `AutomationEvent`s are implemented on the returned `AutomationEventList`
//     /// # Panics
//     /// Panics if the given string contains null characters
//     /// # Examples
//     /// ```
//     /// # use raylib::{Raylib, Color};
//     /// # let mut rl = Raylib::init_window(100, 100, "Automation events", 60);
//     /// let mut auto_events = rl.load_automation_event_list_empty();
//     /// rl.set_automation_event_list(&mut auto_events);
//     /// while !rl.window_should_close() {
//     ///     if rl.is_automation_event_list_set() {
//     ///         rl.start_automation_event_recording();
//     ///     } else { 
//     ///         for &event in auto_events.iter() {
//     ///             rl.play_automation_event(event);
//     ///         }
//     ///     }
//     ///     let draw = rl.begin_drawing();
//     ///     draw.clear_background(Color::RAYWHITE);
//     ///     draw.end();
//     ///     // # break
//     /// }
//     /// rl.stop_automation_event_recording();
//     /// ```
//     pub fn load_automation_event_list(&mut self, filename: &str) -> AutomationEventList {
//         self.load_automation_event_list_cstr(CString::new(filename).expect("expected a string without null characters").as_c_str())
//     }
//
//     /// Load automation events list from file
//     /// Other methods related to `AutomationEvent`s are implemented on the returned `AutomationEventList`
//     pub fn load_automation_event_list_cstr(&mut self, filename: &CStr) -> AutomationEventList {
//         let list = unsafe { ffi::LoadAutomationEventList(filename.as_ptr()) };
//         AutomationEventList(list)
//     }
//
//     /// Creates an empty automation events list
//     pub fn load_automation_event_list_empty(&mut self) -> AutomationEventList {
//         let list = unsafe { ffi::LoadAutomationEventList(std::ptr::null()) };
//         AutomationEventList(list)
//     }
//
//     /// Sets this automation event list as the one to be recorded to
//     pub fn set_automation_event_list(&mut self, list: &mut AutomationEventList) {
//         self.automation_event_set = true;
//         unsafe { ffi::SetAutomationEventList(&mut list.0 as *mut _) };
//     }
//
//     /// Checks if there is an automation event list currently set
//     pub fn is_automation_event_list_set(&self) -> bool {
//         self.automation_event_set
//     }
//
//     /// Checks if we are currently recording to an automation event list
//     pub fn is_automation_event_list_recording(&self) -> bool {
//         self.automation_event_recording
//     }
//
//     /// Start recording automation events
//     /// Returns an error and does nothing if an automation event list was not set or if a recording was already launched
//     pub fn start_automation_event_recording(&mut self) -> Result<(), ()> {
//         if !self.automation_event_set || self.automation_event_recording {
//             return Err(())
//         } 
//         self.automation_event_recording = true;
//
//         unsafe { ffi::StartAutomationEventRecording() };
//         Ok(())
//     }
//
//     /// Stops recording automation events
//     /// Returns an error and does nothing if a recording wasn't started
//     pub fn stop_automation_event_recording(&mut self) -> Result<(), ()> {
//         if !self.automation_event_recording {
//             return Err(())
//         }
//         self.automation_event_recording = false;
//
//         unsafe { ffi::StopAutomationEventRecording() };
//         Ok(())
//     }
//
//     /// Play a recorded automation event
//     pub fn play_automation_event(&mut self, event: AutomationEvent) {
//         unsafe { ffi::PlayAutomationEvent(event) }
//     }
// }
//
// impl AutomationEventList {
//     /// Exports automation events list to text file
//     /// Returns Ok(()) on success and Err(()) if the data couldn't be saved, or if raylib wasn't built with automation event support
//     /// # Panics
//     /// Panics if the given string contains null characters
//     pub fn export(&self, filename: &str) -> Result<(), ()> {
//         self.export_cstr(CString::new(filename).expect("expected a string without null characters").as_c_str())
//     }
//
//     /// Exports automation events list to text file
//     /// Returns Ok(()) on success and Err(()) if the data couldn't be saved, or if raylib wasn't built with automation event support
//     pub fn export_cstr(&self, filename: &CStr) -> Result<(), ()> {
//         let success = unsafe { ffi::ExportAutomationEventList(self.0, filename.as_ptr()) };
//         if success { Ok(()) } else { Err(()) }
//     }
// }
