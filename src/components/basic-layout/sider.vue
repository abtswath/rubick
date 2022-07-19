<template>
    <NLayoutSider collapsed bordered collapse-mode="width" :collapsed-width="60">
        <div class="menu">
            <div class="menu-top">
                <NMenu :value="active" @update:value="updateValue" :root-indent="0" :options="options" collapsed
                    :collapsed-width="60" :collapsed-icon-size="32" />
            </div>
            <div class="menu-bottom">
                <NMenu :value="active" @update:value="updateValue" :root-indent="0" :options="bottomOptions" collapsed
                    :collapsed-width="60" :collapsed-icon-size="32" />
            </div>
        </div>
    </NLayoutSider>
</template>

<script lang="ts" setup>
import { NLayoutSider, NMenu } from 'naive-ui';
import useMenu from '@/compositions/use-menu';

const { options, bottomOptions, active, updateValue } = useMenu();
</script>

<style lang="scss" scoped>
.menu {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-direction: column;
    height: 100%;

    &-top {
        .icon {
            height: 32px;
            background: #c6c6c6;
        }
    }

    &-bottom {}
}
</style>
