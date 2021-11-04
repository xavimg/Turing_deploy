import { TGame } from './TGame';
import { TServerI } from './server/TServerI';
import { TMockServer } from './server/TMockServer';

export const main = () => {
  let server : TServerI = new TMockServer;
  let game = new TGame(server).updateSystem().withConstructedGraphics();
  document.body.appendChild(game.app.view);
  debugger;
}