import { invoke, InvokeArgs } from '@tauri-apps/api/tauri';

export interface Request {
    request<T, D extends {} = {}>(url: string, data?: D): Promise<T>;
}

export class TauriAdapter implements Request {
    public request<T>(command: string, data?: InvokeArgs): Promise<T> {
        return invoke<T>(command, data);
    }
}

type Adapter = 'tauri';

class Factory {

    private adapters: { [key in Adapter]: () => Request } = {
        'tauri': this.createTauriAdapter
    };

    protected adapter: Adapter;

    constructor(adapter: Adapter) {
        this.adapter = adapter;
    }

    public make(): Request {
        return this.adapters[this.adapter]();
    }

    protected createTauriAdapter(): TauriAdapter {
        return new TauriAdapter();
    }
}

export default new Factory('tauri').make();
