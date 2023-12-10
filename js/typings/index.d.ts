declare namespace Deno {
    const internal: unique symbol;
    function inspect(value: unknown, options?: InspectOptions): string
}

declare interface InspectOptions {
    depth: number
}

declare interface Deno {
    [Deno.internal]: {
        core: {
            opAsync: (op: string) => Promise<PluginEvent>
            ops: InternalApi
        }
    };
}

declare type PluginEvent = ViewEvent | ViewCreated | ViewDestroyed | PluginCommand

declare type ViewEvent = {
    type: "ViewEvent"
    eventName: string
    widget: Instance
}

declare type ViewCreated = {
    type: "ViewCreated"
    reconcilerMode: string
    viewName: string
}

declare type ViewDestroyed = {
    type: "ViewDestroyed"
}

declare type PluginCommand = {
    type: "PluginCommand"
    commandType: "stop"
}

declare type UiWidget = {}

declare type ComponentType = string;
declare type Props = { children?: any } & { [key: string]: any };

declare type Root = Instance
declare type Instance = UiWidget
declare type TextInstance = UiWidget
declare type ChildSet = (Instance | TextInstance)[]
declare type UpdatePayload = string[];

type SuspenseInstance = never;

declare interface InternalApi {
    op_log_trace(target: string, message: string): void;
    op_log_debug(target: string, message: string): void;
    op_log_info(target: string, message: string): void;
    op_log_warn(target: string, message: string): void;
    op_log_error(target: string, message: string): void;

    op_react_call_event_listener(instance: Instance, eventName: string): void;

    op_react_get_root(): Root;

    op_react_create_instance(type: ComponentType, props: Props): Instance;

    op_react_create_text_instance(text: string): TextInstance;

    op_react_append_child(parent: Instance, child: Instance | TextInstance): void;

    op_react_call_event_listener(instance: Instance, eventName: string): void;

    // mutation mode
    op_react_remove_child(parent: Instance, child: Instance | TextInstance): void;

    op_react_insert_before(
        parent: Instance,
        child: Instance | TextInstance | SuspenseInstance,
        beforeChild: Instance | TextInstance | SuspenseInstance
    ): void;

    op_react_set_properties(instance: Instance, properties: Props): void;

    op_react_set_text(instance: Instance, text: string): void;

    // persistent mode
    op_react_clone_instance(instance: Instance, updatePayload: UpdatePayload, type: ComponentType, oldProps: Props, newProps: Props, keepChildren: boolean): Instance;

    op_react_replace_container_children(container: Instance, newChildren: ChildSet): void;
}
