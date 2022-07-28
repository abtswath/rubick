<template>
    <NLayout class="basic-layout" has-sider>
        <Sider />
        <NLayoutContent :content-style="{ padding: '8px' }" :native-scrollbar="false">
            <RouterView v-slot="{ Component }">
                <KeepAlive :include="keepalivePages">
                    <component :is="Component" />
                </KeepAlive>
            </RouterView>
        </NLayoutContent>
    </NLayout>
</template>

<script lang="ts" setup>
import { NLayout, NLayoutContent } from 'naive-ui';
import Sider from './sider.vue';
import useKeepalive from '@/compositions/use-keepalive';

const keepalivePages = useKeepalive();
</script>

<style lang="scss" scoped>
.basic-layout {
    height: 100%;
    padding-top: 32px;
    box-sizing: border-box;
}
</style>
