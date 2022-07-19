<template>
    <NConfigProvider tag="main" :theme-overrides="theme">
        <NDialogProvider>
            <NMessageProvider>
                <RouterView />
            </NMessageProvider>
        </NDialogProvider>
    </NConfigProvider>
</template>
<script setup lang="ts">
import { NConfigProvider, GlobalThemeOverrides, NDialogProvider, NMessageProvider } from 'naive-ui';
const theme: GlobalThemeOverrides = {
    common: {
        primaryColor: '#00876f',
        primaryColorHover: '#03a387',
        primaryColorPressed: '#006b57',
        primaryColorSuppl: '#03A387',
        infoColor: '#637c74',
        infoColorHover: '#7d968d',
        infoColorPressed: '#4b635b',
        infoColorSuppl: '#7D968D',
        successColor: '#5a7b90',
        successColorHover: '#7495ab',
        successColorSuppl: '#7495AB',
        successColorPressed: '#416277',
        warningColor: '#747876',
        warningColorHover: '#8e918f',
        warningColorSuppl: '#8e918f',
        warningColorPressed: '#5c5f5d',
        errorColor: '#de3730',
        errorColorHover: '#ff5449',
        errorColorSuppl: '#ff5449',
        errorColorPressed: '#ba1a1a',
        baseColor: '#fefcfa',
        textColorBase: '#000000',
        textColor1: '#1B1C1B',
        textColor2: '#454746',
        textColorDisabled: 'rgba(194, 194, 194, 1)',
        borderColor: '#757876',
    }
};

</script>
<style>
</style>
