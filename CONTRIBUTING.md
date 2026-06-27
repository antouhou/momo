This is a set of guidelines and rules for contributing to the project. They are intended to make collaboration easier 
by standardizing code conventions and such.

## General rules to follow when writing code:

1. Use `thiserror` for error handling.
2. Don't make files too large. If a file exceeds around 500 lines, consider splitting it into smaller modules.
3. Create tests in a separate `tests` file. Writing tests in the same file makes it a bit harder to review.
4. Run `cargo clippy` and `cargo fmt --fix` before commiting.
5. When designing module structure or a new feature, try to use domain-driven approach. For example, if you need a 
controller for whatever functionality you are implementing, don't put it into the controllers module, but create a 
submodule for that functionality. For example, Bluetooth controller should be in `bluetooth/controller.rs`, not in 
`controllers/bluetooth_controller.rs`. This makes code much more structured and easier to navigate. You don't have 
to think "oh I want to work on controllers" when you're working on Bluetooth, you just go to the Bluetooth module 
and see all the code related to it.

## Rules for UI development:

1. When working on UI, always put new components in a separate module and create a style.rs file for it.
2. Reuse colors and constants as much as possible. If you notice that a particular constant would benefit from
lifting it up in the module tree, do so.
3. Do not create new values for paddings and spacings unless absolutely necessary. Use existing ones to keep the app
consistent.
4. Do not use inline values for paddings, font sizes, colors, spacings, etc. Store the actual value somewhere, so the 
app style is consistent and can be changed by changing a single value.
5. Keep all styles and animations consistent with the rest of the project.
6. When working with focus, try to avoid using focus keys as much as possible. Use them only when absolutely necessary, 
in extremely rare cases, when there is absolutely no other way to achieve the desired result. In all other cases, rely 
on the spatial navigation, scopes, and such. Build a correct topology first.
7. When designing a new component, keep in mind that all spacings, sizes, and padding should be uniform and rhythm with 
the rest of the project. There should be multiple levels/layers of spacings and paddings, of course, but they all should
rhythm nonetheless.
8. NEVER manually calculate sizes of ANYTHING outside the most basic components, such as buttons.
9. Never block UI thread or do any system calls in it. If you need to do something that may block in the UI code, just
pass a state handle to the thread that does the blocking work, and let it update the state when needed. Communicate
with that thread over usual std channels.
10. Try to make components as generic as possible. For example, button should not know about the context it's in - it's 
not button's responsibility. Use channels to communicate to the parent that button state has changed - for example that 
it has been activated, clicked or so on, and let the parent decide what to do with it.