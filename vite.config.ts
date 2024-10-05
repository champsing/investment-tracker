import { defineConfig } from "vite";
import { fileURLToPath, URL } from 'url';
import vue from "@vitejs/plugin-vue";

export default defineConfig({
    plugins: [vue()],
    css: {
        preprocessorOptions: {
            scss: {
                api: 'modern-compiler'
            }
        }
    },
    resolve: {
        alias: {
            '@': fileURLToPath(new URL('./web', import.meta.url)),
            '@assets': fileURLToPath(new URL('./web/assets', import.meta.url)),
            '@components': fileURLToPath(new URL('./web/components', import.meta.url)),
            '@composables': fileURLToPath(new URL('./web/composables', import.meta.url)),
        }
    },
    build: {
        rollupOptions: {
            output: {
                manualChunks(id: string) {
                    if (id.indexOf("node_modules/") != -1) {
                        return id.split("node_modules/")[1].split("/")[0]
                    }
                },
            },
        },
    },
});
