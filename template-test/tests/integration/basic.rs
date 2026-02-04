use anyhow::Result;

use crate::{CargoGenerate, Style};

macro_rules! template_test {
    ($name:ident, $config:expr) => {
        #[tokio::test]
        async fn $name() -> Result<()> {
            $config
                .build()
                .await?
                .tests(concat!(stringify!($name), "_template"))
                .await
        }
    };
}

template_test!(default_template, CargoGenerate::default());

template_test!(
    all_feature_template,
    CargoGenerate {
        websocket: true,
        tracing: true,
        style: Style::Unocss,
        docker: true,
        cucumber: true,
    }
);

template_test!(
    websocket_only,
    CargoGenerate {
        websocket: true,
        ..Default::default()
    }
);

template_test!(
    tracing_only,
    CargoGenerate {
        tracing: true,
        ..Default::default()
    }
);

template_test!(
    style_unocss_only,
    CargoGenerate {
        style: Style::Unocss,
        ..Default::default()
    }
);

template_test!(
    docker_only,
    CargoGenerate {
        docker: true,
        ..Default::default()
    }
);

template_test!(
    cucumber_only,
    CargoGenerate {
        cucumber: true,
        ..Default::default()
    }
);

// Websocket with test
template_test!(
    websocket_and_cucumber_only,
    CargoGenerate {
        websocket: true,
        ..Default::default()
    }
);
