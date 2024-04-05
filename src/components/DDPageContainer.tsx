interface Props {
  children: JSX.Element;
}

export default function DDPageContainer({ children }: Props) {
  return (
    <main className="border-x-2 border-dashed flex justify-center h-screen">
      {children}
    </main>
  );
}
