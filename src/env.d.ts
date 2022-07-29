/// <reference types="vite/client" />

import { Store } from 'vuex';
import { SearchHistoryState } from './store/modules/search-history';
import { WindowState } from './store/modules/window';

declare module '*.vue' {
    import type { DefineComponent } from 'vue'
    // eslint-disable-next-line @typescript-eslint/no-explicit-any, @typescript-eslint/ban-types
    const component: DefineComponent<{}, {}, any>
    export default component
}

declare module '@vue/runtime-core' {
    interface State {
        window: WindowState
        searchHistory: SearchHistoryState
    }

    interface ComponentCustomProperties {
        $store: Store<State>
    }
}
