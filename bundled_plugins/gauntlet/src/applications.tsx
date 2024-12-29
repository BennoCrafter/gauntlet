import { GeneratedCommand, GeneratorProps } from "@project-gauntlet/api/helpers";
import { walk, WalkOptions } from "@std/fs/walk";
import { debounce } from "@std/async/debounce";
import { current_os, wayland } from "gauntlet:bridge/internal-all";
import { linux_app_from_path, linux_application_dirs, linux_open_application, } from "gauntlet:bridge/internal-linux";
import {
    macos_app_from_arbitrary_path,
    macos_app_from_path,
    macos_application_dirs,
    macos_major_version,
    macos_open_application,
    macos_open_setting_13_and_post,
    macos_open_setting_pre_13,
    macos_settings_13_and_post,
    macos_settings_pre_13,
    macos_system_applications
} from "gauntlet:bridge/internal-macos";
import { applicationAccessories, applicationActions, OpenWindowData } from "./window/shared";
import { applicationEventLoopX11 } from "./window/x11";

export default async function Applications({ add, remove, get, getAll }: GeneratorProps): Promise<void | (() => void)> {
    const openWindows: Record<string, OpenWindowData> = {};

    switch (current_os()) {
        case "linux": {
            const cleanup = await genericGenerator<LinuxDesktopApplicationData>(
                linux_application_dirs(),
                path => linux_app_from_path(path),
                (id, data) => {
                    if (wayland()) {
                        // TODO
                        return {
                            name: data.name,
                            actions: [
                                {
                                    label: "Open application",
                                    run: () => {
                                        linux_open_application(id)
                                    },
                                }
                            ],
                            accessories: applicationAccessories(id, openWindows),
                            icon: data.icon,
                            "__linux__": {
                                startupWmClass: data.startup_wm_class
                            }
                        }
                    } else {
                        return {
                            name: data.name,
                            actions: applicationActions(
                                id,
                                () => {
                                    linux_open_application(id)
                                },
                                (windowId: string) => {
                                    // TODO
                                    console.log(`focusing window: ${windowId}`)
                                },
                                openWindows
                            ),
                            accessories: applicationAccessories(id, openWindows),
                            icon: data.icon, // TODO lazy icons
                            "__linux__": {
                                startupWmClass: data.startup_wm_class
                            }
                        }
                    }
                },
                add,
                remove,
            );

            if (wayland()) {
                // TODO
            } else {
                applicationEventLoopX11(
                    openWindows,
                    (windowId: string) => {
                        console.log(`focusing window: ${windowId}`)
                    },
                    add,
                    get,
                    getAll
                );
            }

            return cleanup;
        }
        case "macos": {
            const majorVersion = macos_major_version();

            if (majorVersion >= 13) {
                for (const setting of macos_settings_13_and_post()) {
                    add(`settings:${setting.preferences_id}`, {
                        name: setting.name,
                        actions: [
                            {
                                label: "Open settings",
                                run: () => {
                                    macos_open_setting_13_and_post(setting.preferences_id)
                                },
                            }
                        ],
                        icon: setting.icon,
                    })
                }
            } else {
                for (const setting of macos_settings_pre_13()) {
                    add(`settings:${setting.path}`, {
                        name: setting.name,
                        actions: [
                            {
                                label: "Open settings",
                                run: () => {
                                    macos_open_setting_pre_13(setting.path)
                                },
                            }
                        ],
                        icon: setting.icon,
                    })
                }
            }

            for (const path of macos_system_applications()) {
                const app = await macos_app_from_path(path)
                if (app) {
                    switch (app.type) {
                        case "add": {
                            let data = app.data;
                            add(data.path, {
                                name: data.name,
                                actions: [
                                    {
                                        label: "Open application",
                                        run: () => {
                                            macos_open_application(data.path)
                                        },
                                    }
                                ],
                                icon: data.icon,
                            })
                            break;
                        }
                    }
                } else {
                    console.error(`System application '${path}' was not loaded`)
                }
            }

            return await genericGenerator<MacOSDesktopApplicationData>(
                macos_application_dirs(),
                path => macos_app_from_arbitrary_path(path),
                (_id, data) => ({
                    name: data.name,
                    actions: [
                        {
                            label: "Open application",
                            run: () => {
                                macos_open_application(data.path)
                            },
                        }
                    ],
                    icon: data.icon,
                }),
                add,
                remove,
                { exts: ["app"], maxDepth: 2 }
            );
        }
    }
}

async function genericGenerator<DATA>(
    directoriesToWatch: string[],
    appFromPath: (path: string) => Promise<undefined | DesktopPathAction<DATA>>,
    commandFromApp: (id: string, data: DATA) => GeneratedCommand,
    add: (id: string, data: GeneratedCommand) => void,
    remove: (id: string) => void,
    walkOpts?: WalkOptions
): Promise<() => void> {
    const paths = directoriesToWatch
        .filter(path => {
            try {
                Deno.lstatSync(path)
                return true
            } catch (err) {
                // most frequent error here is NotFound
                return false
            }
        });

    for (const path of paths) {
        for await (const dirEntry of walk(path, walkOpts)) {
            const app = await appFromPath(dirEntry.path);
            if (app) {
                switch (app.type) {
                    case "add": {
                        add(app.id, commandFromApp(app.id, app.data))
                        break;
                    }
                }
            }
        }
    }

    const watcher = Deno.watchFs(paths);

    const handle = debounce(
        async (event: Deno.FsEvent) => {
            switch (event.kind) {
                case "create":
                case "modify":
                case "remove": {
                    for (const path of event.paths) {
                        const app = await appFromPath(path);
                        if (app) {
                            switch (app.type) {
                                case "remove": {
                                    remove(app.id)
                                    break;
                                }
                                case "add": {
                                    add(app.id, commandFromApp(app.id, app.data))
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        },
        1000
    );

    // noinspection ES6MissingAwait
    (async () => {
        for await (const event of watcher) {
            handle(event)
        }
    })();

    return () => {
        watcher.close()
    }
}


