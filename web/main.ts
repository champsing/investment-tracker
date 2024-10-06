import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";
import { createVuestic, createIconsConfig } from "vuestic-ui";

createApp(App)
    .use(createVuestic({
        config: {
            icons: createIconsConfig({
                aliases: [],
                fonts: [
                    {
                        name: 'ms-{icon}',
                        resolve: ({ icon }: any) => ({
                            class: 'material-symbols-rounded',
                            content: icon,
                            tag: 'span',
                        })
                    }
                ]
            }),
        },
    }))
    .mount("#app");
