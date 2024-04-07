interface Props {
  children: JSX.Element | JSX.Element[];
}

export default function DDPageContainer({ children }: Props) {
  return (
    <main className="flex flex-col justify-center min-h-screen py-4">
      {children}
    </main>
  );
}
