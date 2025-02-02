use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;

use anyhow::anyhow;
use anyhow::Context;
use deno_core::op2;
use deno_core::serde_v8;
use deno_core::v8;
use deno_core::OpState;
use futures::executor::block_on;
use gauntlet_common::model::ActionPanelSectionWidget;
use gauntlet_common::model::ActionPanelSectionWidgetOrderedMembers;
use gauntlet_common::model::ActionPanelWidget;
use gauntlet_common::model::ActionPanelWidgetOrderedMembers;
use gauntlet_common::model::ActionWidget;
use gauntlet_common::model::CheckboxWidget;
use gauntlet_common::model::CodeBlockWidget;
use gauntlet_common::model::ContentWidget;
use gauntlet_common::model::ContentWidgetOrderedMembers;
use gauntlet_common::model::DatePickerWidget;
use gauntlet_common::model::DetailWidget;
use gauntlet_common::model::EmptyViewWidget;
use gauntlet_common::model::EntrypointId;
use gauntlet_common::model::FormWidget;
use gauntlet_common::model::FormWidgetOrderedMembers;
use gauntlet_common::model::GridItemWidget;
use gauntlet_common::model::GridSectionWidget;
use gauntlet_common::model::GridSectionWidgetOrderedMembers;
use gauntlet_common::model::GridWidget;
use gauntlet_common::model::GridWidgetOrderedMembers;
use gauntlet_common::model::H1Widget;
use gauntlet_common::model::H2Widget;
use gauntlet_common::model::H3Widget;
use gauntlet_common::model::H4Widget;
use gauntlet_common::model::H5Widget;
use gauntlet_common::model::H6Widget;
use gauntlet_common::model::HorizontalBreakWidget;
use gauntlet_common::model::IconAccessoryWidget;
use gauntlet_common::model::ImageLike;
use gauntlet_common::model::ImageSource;
use gauntlet_common::model::ImageSourceAsset;
use gauntlet_common::model::ImageSourceUrl;
use gauntlet_common::model::ImageWidget;
use gauntlet_common::model::InlineSeparatorWidget;
use gauntlet_common::model::InlineWidget;
use gauntlet_common::model::InlineWidgetOrderedMembers;
use gauntlet_common::model::ListItemAccessories;
use gauntlet_common::model::ListItemWidget;
use gauntlet_common::model::ListSectionWidget;
use gauntlet_common::model::ListSectionWidgetOrderedMembers;
use gauntlet_common::model::ListWidget;
use gauntlet_common::model::ListWidgetOrderedMembers;
use gauntlet_common::model::MetadataIconWidget;
use gauntlet_common::model::MetadataLinkWidget;
use gauntlet_common::model::MetadataSeparatorWidget;
use gauntlet_common::model::MetadataTagItemWidget;
use gauntlet_common::model::MetadataTagListWidget;
use gauntlet_common::model::MetadataTagListWidgetOrderedMembers;
use gauntlet_common::model::MetadataValueWidget;
use gauntlet_common::model::MetadataWidget;
use gauntlet_common::model::MetadataWidgetOrderedMembers;
use gauntlet_common::model::ParagraphWidget;
use gauntlet_common::model::PasswordFieldWidget;
use gauntlet_common::model::PhysicalKey;
use gauntlet_common::model::PluginId;
use gauntlet_common::model::RootWidget;
use gauntlet_common::model::RootWidgetMembers;
use gauntlet_common::model::SearchBarWidget;
use gauntlet_common::model::SelectItemWidget;
use gauntlet_common::model::SelectWidget;
use gauntlet_common::model::SelectWidgetOrderedMembers;
use gauntlet_common::model::SeparatorWidget;
use gauntlet_common::model::TextAccessoryWidget;
use gauntlet_common::model::TextFieldWidget;
use gauntlet_common::model::UiPropertyValue;
use gauntlet_common::model::UiRenderLocation;
use gauntlet_common::model::UiWidgetId;
use gauntlet_common::model::WidgetVisitor;
use gauntlet_component_model::Component;
use gauntlet_component_model::Component::Root;
use gauntlet_component_model::Property;
use gauntlet_component_model::PropertyType;
use gauntlet_component_model::SharedType;
use indexmap::IndexMap;
use serde::de;
use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use tokio::runtime::Handle;

use crate::api::BackendForPluginRuntimeApi;
use crate::api::BackendForPluginRuntimeApiProxy;
use crate::component_model::ComponentModel;
use crate::model::JsUiRenderLocation;
use crate::plugin_data::PluginData;

#[op2]
pub fn show_plugin_error_view(
    state: Rc<RefCell<OpState>>,
    #[string] entrypoint_id: String,
    #[serde] render_location: JsUiRenderLocation,
) -> anyhow::Result<()> {
    let api = {
        let state = state.borrow();

        let api = state.borrow::<BackendForPluginRuntimeApiProxy>().clone();

        api
    };

    let render_location = match render_location {
        JsUiRenderLocation::InlineView => UiRenderLocation::InlineView,
        JsUiRenderLocation::View => UiRenderLocation::View,
    };

    tokio::spawn(async move {
        api.ui_show_plugin_error_view(EntrypointId::from_string(entrypoint_id), render_location)
            .await
    });

    Ok(())
}

#[op2(fast)]
pub fn show_preferences_required_view(
    state: Rc<RefCell<OpState>>,
    #[string] entrypoint_id: String,
    plugin_preferences_required: bool,
    entrypoint_preferences_required: bool,
) -> anyhow::Result<()> {
    let api = {
        let state = state.borrow();

        let api = state.borrow::<BackendForPluginRuntimeApiProxy>().clone();

        api
    };

    tokio::spawn(async move {
        api.ui_show_preferences_required_view(
            EntrypointId::from_string(entrypoint_id),
            plugin_preferences_required,
            entrypoint_preferences_required,
        )
        .await
    });

    Ok(())
}

#[op2(fast)]
pub fn clear_inline_view(state: Rc<RefCell<OpState>>) -> anyhow::Result<()> {
    let api = {
        let state = state.borrow();

        let api = state.borrow::<BackendForPluginRuntimeApiProxy>().clone();

        api
    };

    tokio::spawn(async move { api.ui_clear_inline_view().await });

    Ok(())
}

#[op2]
#[string]
pub fn op_inline_view_entrypoint_id(state: Rc<RefCell<OpState>>) -> Option<String> {
    state
        .borrow()
        .borrow::<PluginData>()
        .inline_view_entrypoint_id()
        .clone()
}

#[op2]
#[serde]
pub fn op_entrypoint_names(state: Rc<RefCell<OpState>>) -> HashMap<String, String> {
    state.borrow().borrow::<PluginData>().entrypoint_names().clone()
}

#[op2]
pub fn op_react_replace_view<'a>(
    scope: &mut v8::HandleScope,
    state: Rc<RefCell<OpState>>,
    #[serde] render_location: JsUiRenderLocation,
    top_level_view: bool,
    #[string] entrypoint_id: &str,
    #[string] entrypoint_name: &str,
    #[serde] container: serde_v8::Value<'a>,
) -> anyhow::Result<()> {
    tracing::trace!(target = "renderer_rs", "Calling op_react_replace_view...");

    let mut deserializer = serde_v8::Deserializer::new(scope, container.v8_value, None);

    let container = RootWidget::deserialize(&mut deserializer)?;

    let entrypoint_id = EntrypointId::from_string(entrypoint_id);
    let entrypoint_name = entrypoint_name.to_string();

    let (api, outer_handle) = {
        let state = state.borrow();

        let api = state.borrow::<BackendForPluginRuntimeApiProxy>().clone();

        let outer_handle = state.borrow::<Handle>().clone();

        (api, outer_handle)
    };

    let render_location = match render_location {
        JsUiRenderLocation::InlineView => UiRenderLocation::InlineView,
        JsUiRenderLocation::View => UiRenderLocation::View,
    };

    block_on(async move {
        outer_handle
            .spawn(async move {
                api.ui_render(
                    entrypoint_id,
                    entrypoint_name,
                    render_location,
                    top_level_view,
                    container,
                )
                .await
            })
            .await
    })??;

    Ok(())
}

#[op2]
#[serde]
pub fn op_component_model(state: Rc<RefCell<OpState>>) -> HashMap<String, Component> {
    state.borrow().borrow::<ComponentModel>().components().clone()
}

#[op2(async)]
#[string]
pub async fn fetch_action_id_for_shortcut(
    state: Rc<RefCell<OpState>>,
    #[string] entrypoint_id: String,
    #[string] key: String,
    modifier_shift: bool,
    modifier_control: bool,
    modifier_alt: bool,
    modifier_meta: bool,
) -> anyhow::Result<Option<String>> {
    let api = {
        let state = state.borrow();

        let api = state.borrow::<BackendForPluginRuntimeApiProxy>().clone();

        api
    };

    let result = api
        .ui_get_action_id_for_shortcut(
            EntrypointId::from_string(entrypoint_id),
            key,
            modifier_shift,
            modifier_control,
            modifier_alt,
            modifier_meta,
        )
        .await?;

    Ok(result)
}

#[op2(async)]
pub async fn show_hud(state: Rc<RefCell<OpState>>, #[string] display: String) -> anyhow::Result<()> {
    let api = {
        let state = state.borrow();

        let api = state.borrow::<BackendForPluginRuntimeApiProxy>().clone();

        api
    };

    api.ui_show_hud(display).await
}

#[op2(async)]
pub async fn hide_window(state: Rc<RefCell<OpState>>) -> anyhow::Result<()> {
    let api = {
        let state = state.borrow();

        let api = state.borrow::<BackendForPluginRuntimeApiProxy>().clone();

        api
    };

    api.ui_hide_window().await
}

#[op2(async)]
pub async fn update_loading_bar(
    state: Rc<RefCell<OpState>>,
    #[string] entrypoint_id: String,
    show: bool,
) -> anyhow::Result<()> {
    let api = {
        let state = state.borrow();

        let api = state.borrow::<BackendForPluginRuntimeApiProxy>().clone();

        api
    };

    api.ui_update_loading_bar(EntrypointId::from_string(entrypoint_id), show)
        .await
}

#[allow(unused)]
fn debug_object_to_json(scope: &mut v8::HandleScope, val: v8::Local<v8::Value>) -> String {
    let local = scope.get_current_context();
    let global = local.global(scope);
    let json_string = v8::String::new(scope, "Deno").expect("Unable to create Deno string");
    let json_object = global
        .get(scope, json_string.into())
        .expect("Global Deno object not found");
    let json_object: v8::Local<v8::Object> = json_object.try_into().expect("Deno value is not an object");
    let inspect_string = v8::String::new(scope, "inspect").expect("Unable to create inspect string");
    let inspect_object = json_object
        .get(scope, inspect_string.into())
        .expect("Unable to get inspect on global Deno object");
    let stringify_fn: v8::Local<v8::Function> = inspect_object.try_into().expect("inspect value is not a function");
    let undefined = v8::undefined(scope).into();

    let json_object = stringify_fn
        .call(scope, undefined, &[val])
        .expect("Unable to get serialize prop");
    let json_string: v8::Local<v8::String> = json_object.try_into().expect("result is not a string");

    let result = json_string.to_rust_string_lossy(scope);

    result
}
