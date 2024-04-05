interface Props {
  children: JSX.Element;
}

export default function DDPageContainer({ children }: Props) {
  return (
    <main className="border-x-2 border-dashed flex flex-col justify-center py-4">
      {children}
    </main>
  );
}
