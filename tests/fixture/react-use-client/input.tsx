'use client';

export default function Component() {
  const [count, setCount] = useState(0);
  useEffect(() => {
    console.log(count);
  }, [count]);
  return null;
}

