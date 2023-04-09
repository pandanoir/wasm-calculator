import { ButtonHTMLAttributes, FC, PropsWithChildren, useState } from 'react';
import { createRoot } from 'react-dom/client';
import init, { calc } from 'rust';

// 計算機画面には、数字や演算子が表示される。
// 数字や演算子をクリックすることで、計算式が入力される。
// 計算式を入力した後、「=」ボタンをクリックすることで、計算結果が表示される。
// 計算結果の表示方法は、画面上に表示する。
// エラー処理を実装し、不正な入力があった場合には、エラーメッセージを表示する。
// 以上の仕様を満たすためには、以下のような実装が必要となります。

const Cell: FC<PropsWithChildren<ButtonHTMLAttributes<HTMLButtonElement>>> = ({
  children,
  ...props
}) => (
  <button
    style={{
      border: 'none',
      background: 'transparent',
      borderLeft: '1px solid #999',
      borderBottom: '1px solid #999',
      padding: '3px',
    }}
    {...props}
  >
    {children}
  </button>
);
const Row: FC<PropsWithChildren> = ({ children }) => (
  <div style={{ display: 'contents' }}>{children}</div>
);
const App: FC = () => {
  const [value, setValue] = useState('');
  const [isCalculating, setIsCalculating] = useState(false);
  const startCalculate = async () => {
    setIsCalculating(true);
    await init();
    try {
      console.log(calc(value));
      setValue(`${calc(value)}`);
      setIsCalculating(false);
    } catch {
      alert(`can't calculate formula: invalid formula was given`);
      setIsCalculating(false);
    }
  };
  return (
    <main>
      <div>{isCalculating ? 'calculating...' : `$ ${value}`}</div>
      <div
        style={{
          width: 300,
          textAlign: 'center',
          display: 'grid',
          gridTemplateColumns: '1fr 1fr 1fr 1fr',
          borderRight: '1px solid #999',
          borderTop: '1px solid #999',
          background: '#fcfcfc',
        }}
      >
        {[[...'789/'], [...'456*'], [...'123-'], [...'0.=+']].map(
          (row, index) => (
            <Row key={index}>
              {row.map((s) => (
                <Cell
                  onClick={() => {
                    if (s === '=') {
                      startCalculate();
                      return;
                    }
                    setValue((v) => v + s);
                  }}
                  key={s}
                  disabled={
                    ('+-*/'.includes(s) && '+-*/'.includes(value.slice(-1))) ||
                    (s === '.' && '+-*/.'.includes(value.slice(-1)))
                  }
                >
                  {s}
                </Cell>
              ))}
            </Row>
          )
        )}
      </div>
    </main>
  );
};

const $root = document.querySelector('#root');
if ($root) {
  createRoot($root).render(<App />);
}
