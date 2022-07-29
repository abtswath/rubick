import { appWindow, LogicalSize } from '@tauri-apps/api/window';
import { State } from 'vue';
import { ActionContext, Store } from 'vuex';

export interface WindowState {
    isMaximized: boolean;
    allowMaximize: boolean;
}

const MutationTypes = {
    setIsMaximized: 'setIsMaximized',
    setAllowMaximize: 'setAllowMaximize'
} as const;

const ActionTypes = {
    toggleMaximize: 'toggleMaximize',
    setSize: 'setSize',
    show: 'show',
    maximize: 'maximize',
    minimize: 'minimize',
    unMaximize: 'unMaximize',
    close: 'close',
    setResizable: 'setResizable',
    disableMaximize: 'disableMaximize',
    enableMaximize: 'enableMaximize',
} as const;

export type WindowMutationTree = {
    [MutationTypes.setIsMaximized]<T extends boolean>(state: WindowState, payload: T): void;
    [MutationTypes.setAllowMaximize]<T extends boolean>(state: WindowState, payload: T): void;
}

export type WindowActionTree = {
    [ActionTypes.toggleMaximize](this: Store<State>, injectee: ActionContext<WindowState, State>): Promise<void>;
    [ActionTypes.setSize]<T extends { width: number, height: number }>(this: Store<State>, injectee: ActionContext<WindowState, State>, payload: T): Promise<void>;
    [ActionTypes.show](this: Store<State>, injectee: ActionContext<WindowState, State>): Promise<void>;
    [ActionTypes.maximize](this: Store<State>, injectee: ActionContext<WindowState, State>): Promise<void>;
    [ActionTypes.minimize](this: Store<State>, injectee: ActionContext<WindowState, State>): Promise<void>;
    [ActionTypes.unMaximize](this: Store<State>, injectee: ActionContext<WindowState, State>): Promise<void>;
    [ActionTypes.close](this: Store<State>, injectee: ActionContext<WindowState, State>): Promise<void>;
    [ActionTypes.setResizable]<T extends boolean>(this: Store<State>, injectee: ActionContext<WindowState, State>, payload: T): Promise<void>;
    [ActionTypes.disableMaximize](this: Store<State>, injectee: ActionContext<WindowState, State>): void;
    [ActionTypes.enableMaximize](this: Store<State>, injectee: ActionContext<WindowState, State>): void;
}

const state: () => WindowState = () => ({
    isMaximized: false,
    allowMaximize: true
});

const mutations: WindowMutationTree = {
    [MutationTypes.setIsMaximized](state, value: boolean) {
        state.isMaximized = value;
    },
    [MutationTypes.setAllowMaximize](state, value: boolean) {
        state.allowMaximize = value;
    }
};

const actions: WindowActionTree = {
    async [ActionTypes.toggleMaximize]({ commit, state }) {
        if (!state.allowMaximize) {
            return;
        }
        await appWindow.toggleMaximize();
        commit('setIsMaximized', false);
    },
    async [ActionTypes.setSize](_, { width, height }) {
        const size = new LogicalSize(width, height);
        await appWindow.setMaxSize(size);
        await appWindow.setSize(size);
    },
    async [ActionTypes.show]() {
        await appWindow.show();
    },
    async [ActionTypes.maximize]() {
        await appWindow.maximize();
    },
    async [ActionTypes.minimize]() {
        await appWindow.minimize();
    },
    async [ActionTypes.unMaximize]() {
        await appWindow.unmaximize();
    },
    async [ActionTypes.close]() {
        await appWindow.close();
    },
    async [ActionTypes.setResizable](_, resizable: boolean) {
        await appWindow.setResizable(resizable);
    },
    [ActionTypes.disableMaximize]({ commit }) {
        commit('setAllowMaximize', false);
    },
    [ActionTypes.enableMaximize]({ commit }) {
        commit('setAllowMaximize', true);
    }
};

export default {
    namespaced: true,
    state,
    actions,
    mutations
};
