<template>
    <NLayoutHeader position="absolute" @dblclick.stop="toggleMaximize" data-tauri-drag-region bordered class="header">
        <div class="action">
            <div class="action-button" role="button" @click="minimize">
                <NIcon size="16">
                    <Minimize />
                </NIcon>
            </div>
            <div class="action-button" :disabled="!allowMaximize" role="button" @click="toggleMaximize">
                <NIcon size="16">
                    <Restore v-if="isMaximized" />
                    <Maximize v-else />
                </NIcon>
            </div>
            <div class="action-button close" role="button" @click="close">
                <NIcon size="16">
                    <Close />
                </NIcon>
            </div>
        </div>
    </NLayoutHeader>
</template>

<script lang="ts" setup>
import { NLayoutHeader, NIcon } from 'naive-ui';
import { Close, Minimize, Maximize, Restore } from '@/components/icon';
import { computed } from 'vue';
import { useStore } from '@/store';

const store = useStore();

const isMaximized = computed(() => store.state.window.isMaximized);
const allowMaximize = computed(() => store.state.window.allowMaximize);
const minimize = () => store.dispatch('window/minimize');
const toggleMaximize = () => store.dispatch('window/toggleMaximize');
const close = () => store.dispatch('window/close');

</script>

<style lang="scss" scoped>
.header {
    height: 32px;
    z-index: 99;

    .action {
        float: right;
        display: flex;
        height: 100%;
        align-items: center;
        justify-content: flex-end;

        &-button {
            width: 46px;
            text-align: center;
            height: 100%;
            padding: 8px 0;
            box-sizing: border-box;

            &[disabled=true] {
                opacity: .5;

                &:hover {
                    background-color: transparent;
                }
            }

            &:hover {
                background-color: #e6e6e6;

                &.close {
                    background-color: #de3730;
                    color: #e3e3e3;
                }
            }
        }
    }
}
</style>
