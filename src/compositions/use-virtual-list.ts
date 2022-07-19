import { computed, h, onMounted, onUnmounted, ref, VNode } from 'vue';

export default <T>({
    el,
    renderer,
    rowHeight = 60,
    styles = {}
}: {
    el: string,
    renderer: (data: T) => VNode
    rowHeight?: number,
    styles?: {},
}) => {

    const size = ref(0);
    const offset = ref(0);
    const top = ref(0);

    const VirtualList = (props: { data: T[] }) => {
        const children = computed<VNode[]>(() => {
            return props.data
                .slice(offset.value, offset.value + size.value)
                .map((item, index) => {
                    return h(renderer(item), {
                        style: {
                            position: 'absolute',
                            left: 0,
                            width: '100%',
                            top: `${(offset.value + index) * rowHeight}px`
                        },
                    });
                })
        });

        return h(
            'div',
            {
                style: Object.assign({
                    position: 'relative',
                    height: `${props.data.length * rowHeight}px`
                }, styles)
            },
            children.value
        );
    }

    onMounted(() => {
        const element = document.querySelector(el) as HTMLElement;
        size.value = Math.ceil(element.offsetHeight / rowHeight) + 4;

        const onResize = (event: Event) => {
            console.log(event)
            size.value = Math.ceil(element.offsetHeight / rowHeight) + 4;
        };
        
        window.addEventListener('resize', onResize);
        onUnmounted(() => {
            window.removeEventListener('resize', onResize);
        });
    });

    const handleScroll = (event: Event) => {
        const element = event.target as HTMLElement;
        top.value = element.scrollTop;
        offset.value = Math.floor(top.value / rowHeight);
    }

    return { VirtualList, handleScroll };
}
