// 自定义导入测试
const { x, y } = useMouse();
const debouncedFn = debounce(() => {
  console.log('debounced');
}, 300);

const data = useLocalStorage('key', 'default');

