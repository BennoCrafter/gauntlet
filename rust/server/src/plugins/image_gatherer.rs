use std::collections::HashMap;
use std::io::Read;

use futures::StreamExt;
use gauntlet_common::model::ImageLike;
use gauntlet_common::model::ImageSource;
use gauntlet_common::model::ImageSourceAsset;
use gauntlet_common::model::ImageSourceUrl;
use gauntlet_common::model::RootWidget;
use gauntlet_common::model::UiWidgetId;
use gauntlet_common::model::WidgetVisitor;
use gauntlet_plugin_runtime::BackendForPluginRuntimeApi;

use crate::plugins::js::BackendForPluginRuntimeApiImpl;

pub struct ImageGatherer<'a> {
    api: &'a BackendForPluginRuntimeApiImpl,
    image_sources: HashMap<UiWidgetId, anyhow::Result<Vec<u8>>>,
}

impl<'a> WidgetVisitor for ImageGatherer<'a> {
    async fn image(&mut self, widget_id: UiWidgetId, widget: &ImageLike) {
        if let ImageLike::ImageSource(image_source) = &widget {
            self.image_sources
                .insert(widget_id, get_image_date(&self.api, image_source).await);
        }
    }
}

impl<'a> ImageGatherer<'a> {
    pub async fn run_gatherer(
        api: &'a BackendForPluginRuntimeApiImpl,
        root_widget: &RootWidget,
    ) -> anyhow::Result<HashMap<UiWidgetId, Vec<u8>>> {
        let mut gatherer = Self {
            api,
            image_sources: HashMap::new(),
        };

        gatherer.root_widget(root_widget).await;

        gatherer
            .image_sources
            .into_iter()
            .map(|(widget_id, image)| image.map(|image| (widget_id, image)))
            .collect::<anyhow::Result<_>>()
    }
}

async fn get_image_date(api: &BackendForPluginRuntimeApiImpl, source: &ImageSource) -> anyhow::Result<Vec<u8>> {
    match source {
        ImageSource::ImageSourceAsset(ImageSourceAsset { asset }) => {
            let bytes = api.get_asset_data(&asset).await?;

            Ok(bytes)
        }
        ImageSource::ImageSourceUrl(ImageSourceUrl { url }) => {
            // FIXME implement error handling so it doesn't error whole view
            // TODO implement caching

            let bytes = ureq::get(&url)
                .call()?
                .into_reader()
                .bytes()
                .collect::<std::io::Result<Vec<u8>>>()?
                .into();

            Ok(bytes)
        }
    }
}
