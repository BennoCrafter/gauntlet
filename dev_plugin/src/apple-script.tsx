import { ReactElement, useEffect, useState } from 'react';
import { Detail } from "@project-gauntlet/api/components";
import { AppleScript } from '@project-gauntlet/api/helpers';

export default function DetailView(): ReactElement {
  const [scriptResult, setScriptResult] = useState<string>('');

  useEffect(() => {
    const runScript = async () => {
      const result = await AppleScript.run(`
        tell application "System Events"
          set currentDate to current date
          return "Hello World! Current Time: " & (currentDate as string)
        end tell`);
      setScriptResult(result);
    };
    runScript();
  }, []);

  return (
      <Detail>
          <Detail.Content>
            <Detail.Content.H1>
              {scriptResult}
            </Detail.Content.H1>
          </Detail.Content>
      </Detail>
  );
};
