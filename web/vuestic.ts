import { createVuestic, createIconsConfig } from "vuestic-ui";

let vuestic = createVuestic({
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
})

export default vuestic;
