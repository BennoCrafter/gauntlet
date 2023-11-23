use std::collections::HashMap;

use anyhow::anyhow;
use zbus::zvariant::Value;

use common::dbus::{DBusUiPropertyContainer, DBusUiPropertyValueType, DBusUiWidget};
use common::model::{EntrypointId, PluginId};

#[derive(Debug, Clone)]
pub struct NativeUiSearchResult {
    pub plugin_id: PluginId,
    pub plugin_name: String,
    pub entrypoint_id: EntrypointId,
    pub entrypoint_name: String,
}

#[derive(Debug, Clone)]
pub enum NativeUiResponseData {
    GetContainer {
        container: NativeUiWidget
    },
    CreateInstance {
        widget: NativeUiWidget
    },
    CreateTextInstance {
        widget: NativeUiWidget
    },
    CloneInstance {
        widget: NativeUiWidget
    },
}

#[derive(Debug, Clone)]
pub enum NativeUiRequestData {
    GetContainer,
    CreateInstance {
        widget_type: String,
        properties: HashMap<String, NativeUiPropertyValue>,
    },
    CreateTextInstance {
        text: String,
    },
    AppendChild {
        parent: NativeUiWidget,
        child: NativeUiWidget,
    },
    CloneInstance {
        widget_type: String,
        properties: HashMap<String, NativeUiPropertyValue>,
    },
    ReplaceContainerChildren {
        container: NativeUiWidget,
        new_children: Vec<NativeUiWidget>,
    },
}

pub type NativeUiWidgetId = u32;

pub fn from_dbus(value: DBusUiPropertyContainer) -> anyhow::Result<HashMap<String, NativeUiPropertyValue>> {
    let result = value.properties
        .into_iter()
        .map(|(key, (value_type, value))| {
            let value = match &(value_type, value.into()) {
                (DBusUiPropertyValueType::String, Value::Str(value)) => NativeUiPropertyValue::String(value.to_string()),
                (DBusUiPropertyValueType::Number, Value::F64(value)) => NativeUiPropertyValue::Number(*value),
                (DBusUiPropertyValueType::Bool, Value::Bool(value)) => NativeUiPropertyValue::Bool(*value),
                (DBusUiPropertyValueType::Function, _) => NativeUiPropertyValue::Function,
                _ => {
                    return Err(anyhow!("invalid type"))
                }
            };

            Ok((key, value))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .collect::<HashMap<_, _>>();

    Ok(result)
}

#[derive(Debug, Clone)]
pub enum NativeUiPropertyValue {
    Function,
    String(String),
    Number(f64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct NativeUiWidget {
    pub widget_id: NativeUiWidgetId,
}

impl From<NativeUiWidget> for DBusUiWidget {
    fn from(value: NativeUiWidget) -> Self {
        Self {
            widget_id: value.widget_id
        }
    }
}

impl From<DBusUiWidget> for NativeUiWidget {
    fn from(value: DBusUiWidget) -> Self {
        Self {
            widget_id: value.widget_id
        }
    }
}
