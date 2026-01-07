function ref(value: any) {
    return {
        value
    };
}
function computed(fn: Function) {
    return fn();
}
const count = ref(0);
const doubled = computed(()=>count.value * 2);

