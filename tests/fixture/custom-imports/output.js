import { useLocalStorage, useMouse } from "@vueuse/core";
import { debounce } from "lodash-es";
const { x, y } = useMouse();
const debouncedFn = debounce(()=>{
    console.log('debounced');
}, 300);
const data = useLocalStorage('key', 'default');

