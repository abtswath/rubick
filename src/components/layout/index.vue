<template>
    <NLayout class="layout" position="absolute">
        <Header />
        <RouterView />
    </NLayout>
</template>

<script lang="ts" setup>
import { NLayout } from 'naive-ui';
import Header from './header.vue';

</script>

<style lang="scss" scoped>
.layout {
    border: 1px solid #dfdfdf;
}
</style>
