use crate::{context::CONTEXT, prelude::*};
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

/// A handle to a view which has been already built into the tree.
pub struct Handle<V> {
    pub(crate) entity: Entity,
    pub(crate) p: PhantomData<V>,
}

// impl<V> DataContext for Handle<V> {
//     fn data<T: 'static>(&self) -> Option<&T> {
//         CONTEXT.with_borrow(|cx| {
//             // Return data for the static model.
//             if let Some(t) = <dyn Any>::downcast_ref::<T>(&()) {
//                 return Some(t);
//             }

//             for entity in self.entity.parent_iter(&cx.tree) {
//                 // Return any model data.
//                 if let Some(model_data_store) = cx.data.get(&entity) {
//                     if let Some(model) = model_data_store.models.get(&TypeId::of::<T>()) {
//                         return model.downcast_ref::<T>();
//                     }
//                 }

//                 // Return any view data.
//                 if let Some(view_handler) = cx.views.get(&entity) {
//                     if let Some(data) = view_handler.downcast_ref::<T>() {
//                         return Some(data);
//                     }
//                 }
//             }

//             None
//         })
//     }
// }

impl<V> Handle<V> {
    /// Returns the [`Entity`] id of the view.
    pub fn entity(&self) -> Entity {
        self.entity
    }

    // /// Returns a mutable reference to the context.
    // pub fn context(&mut self) -> &mut Context {
    //     self.cx
    // }

    pub fn parent(&self) -> Entity {
        CONTEXT.with_borrow(|cx| cx.tree.get_parent(self.entity).unwrap_or(Entity::root()))
    }

    /// Marks the view as being ignored.
    pub(crate) fn ignore(self) -> Self {
        // CONTEXT.with_borrow_mut(|cx| {
        //     cx.tree.set_ignored(self.entity, true);
        //     // self.focusable(false)
        // });

        self
    }

    /// Stop the user from tabbing out of a subtree, which is useful for modal dialogs.
    pub fn lock_focus_to_within(self) -> Self {
        CONTEXT.with_borrow_mut(|cx| {
            cx.tree.set_lock_focus_within(self.entity, true);
            cx.focus_stack.push(cx.focused);
            if !cx.focused.is_descendant_of(&cx.tree, self.entity) {
                let new_focus = vizia_storage::TreeIterator::subtree(&cx.tree, self.entity)
                    .find(|node| {
                        crate::tree::is_navigatable(&cx.tree, &cx.style, *node, Entity::root())
                    })
                    .unwrap_or(cx.focus_stack.pop().unwrap());
                cx.with_current(new_focus, |cx| cx.focus());
            }
        });

        self
    }

    /// Mody the internal data of the view.
    pub fn modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
        V: 'static,
    {
        CONTEXT.with_borrow_mut(|cx| {
            if let Some(view) = cx
                .views
                .get_mut(&self.entity)
                .and_then(|view_handler| view_handler.downcast_mut::<V>())
            {
                (f)(view);
            }
        });

        self
    }

    /// Callback which is run when the view is built/rebuilt.
    pub fn on_build<F>(self, callback: F) -> Self
    where
        F: Fn(&mut EventContext),
    {
        CONTEXT.with_borrow_mut(|cx| {
            let mut event_context = EventContext::new(cx);
            event_context.current = self.entity;
            (callback)(&mut event_context);
        });

        self
    }

    pub fn bind<L, F>(self, lens: L, closure: F) -> Self
    where
        L: Lens,
        <L as Lens>::Target: Data,
        F: 'static + Fn(Handle<V>, L),
    {
        CONTEXT.with_borrow_mut(|cx| {
            let entity = self.entity();
            Binding::new(cx, lens, move |cx, data| {
                let new_handle = Handle { entity, p: Default::default() };

                cx.set_current(new_handle.entity);
                (closure)(new_handle, data);
            });
        });

        self
    }

    /// Marks the view as needing a relayout.
    pub fn needs_relayout(&mut self) {
        CONTEXT.with_borrow_mut(|cx| {
            cx.needs_relayout();
        });
    }

    /// Marks the view as needing a restyle.
    pub fn needs_restyle(&mut self) {
        CONTEXT.with_borrow_mut(|cx| {
            cx.needs_restyle();
        });
    }

    /// Marks the view as needing a redraw.
    pub fn needs_redraw(&mut self) {
        CONTEXT.with_borrow_mut(|cx| {
            cx.needs_redraw();
        });
    }

    /// Returns the bounding box of the view.
    pub fn bounds(&self) -> BoundingBox {
        CONTEXT.with_borrow_mut(|cx| cx.cache.get_bounds(self.entity))
    }

    /// Returns the scale factor of the device.
    pub fn scale_factor(&self) -> f32 {
        CONTEXT.with_borrow_mut(|cx| cx.scale_factor())
    }
}
