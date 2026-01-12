function App() {
  const [count, setCount] = useState(0);
  
  useReactEffect(() => {
    console.log("Effect");
  }, []);
  
  const doubled = useMemoized(() => count * 2, [count]);
  
  return <div>{doubled}</div>;
}
