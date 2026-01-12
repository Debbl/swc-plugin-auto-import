function App() {
  const [count, setCount] = useState(0);
  
  const portal = createPortal(<div>Portal</div>, document.body);
  
  flushSync(() => {
    setCount(1);
  });
  
  return <div>{count}{portal}</div>;
}
