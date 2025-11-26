"use client";

import { useState, useEffect, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { TrendingUp, TrendingDown, ShieldAlert, Coins } from "lucide-react";

// --- TYPES ---
type Creature = {
  id: number;
  name: string;
  country: string;
  sprite: string;
  hp: number;
  maxHp: number;
  alive: boolean;
  color: string;
  borderColor: string;
  userShares: number; // New: Track how many shares user owns
};

// --- CONFIGURATION ---
const INITIAL_STATE: Creature[] = [
  { 
    id: 0, name: "YETI", country: "🇳🇵", sprite: "❄️", 
    hp: 100, maxHp: 100, alive: true, userShares: 0,
    color: "bg-blue-900/40", borderColor: "border-blue-400" 
  },
  { 
    id: 1, name: "MAPINGUARI", country: "🇧🇷", sprite: "🌿", 
    hp: 100, maxHp: 100, alive: true, userShares: 0,
    color: "bg-amber-900/40", borderColor: "border-amber-600" 
  },
  { 
    id: 2, name: "ZMEY", country: "🇷🇺", sprite: "🔥", 
    hp: 100, maxHp: 100, alive: true, userShares: 0,
    color: "bg-red-900/40", borderColor: "border-red-500" 
  },
  { 
    id: 3, name: "NAGA", country: "🇮🇳", sprite: "🐍", 
    hp: 100, maxHp: 100, alive: true, userShares: 0,
    color: "bg-emerald-900/40", borderColor: "border-emerald-400" 
  },
];

export default function Arena() {
  const [creatures, setCreatures] = useState<Creature[]>(INITIAL_STATE);
  const [logs, setLogs] = useState<string[]>([]);
  const [turn, setTurn] = useState(0);
  const [balance, setBalance] = useState(1000); // Mock Wallet Balance
  const logsEndRef = useRef<HTMLDivElement>(null);

  // --- MOCK CRANK (Autonomous Battle) ---
  useEffect(() => {
    const interval = setInterval(() => {
      executeTurn();
    }, 3000); // 3 seconds per turn to allow reading
    return () => clearInterval(interval);
  }, [creatures]);

  // Auto-scroll logs
  useEffect(() => {
    logsEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [logs]);

  // --- NEW BATTLE LOGIC: EVERYONE ATTACKS ---
  const executeTurn = () => {
    // 1. Check if game over
    const aliveCount = creatures.filter(c => c.alive).length;
    if (aliveCount <= 1) return;

    let newLogs: string[] = [];
    // Clone state to modify safely
    let nextState = [...creatures];

    // 2. Loop through every creature
    nextState.forEach((attacker) => {
      if (!attacker.alive) return;

      // Select random target (not self, must be alive)
      const potentialTargets = nextState.filter(c => c.id !== attacker.id && c.alive);
      if (potentialTargets.length === 0) return;

      const target = potentialTargets[Math.floor(Math.random() * potentialTargets.length)];
      
      // Calculate Damage
      const damage = Math.floor(Math.random() * 12) + 5; // 5-17 DMG

      // Apply Damage
      target.hp = Math.max(0, target.hp - damage);
      
      // Log Event
      newLogs.push(`${attacker.name} hits ${target.name} (-${damage})`);

      // Check Death
      if (target.hp === 0 && target.alive) {
        target.alive = false;
        newLogs.push(`💀 ${target.name} ELIMINATED!`);
      }
    });

    setCreatures([...nextState]); // Trigger re-render
    setTurn(prev => prev + 1);
    setLogs(prev => [...prev, ...newLogs]);
  };

  // --- MARKET ACTIONS ---
  const handleTrade = (id: number, type: 'buy' | 'sell') => {
    setCreatures(prev => prev.map(c => {
      if (c.id === id) {
        if (type === 'buy') {
          if (balance < 10) return c; // Check funds
          setBalance(b => b - 10);
          return { ...c, userShares: c.userShares + 1 };
        } else {
          if (c.userShares <= 0) return c; // Check holdings
          setBalance(b => b + 10); // Simplified 1:1 price for mock
          return { ...c, userShares: c.userShares - 1 };
        }
      }
      return c;
    }));
  };

  return (
    <main className="min-h-screen bg-slate-950 text-white p-2 md:p-6 font-mono flex flex-col items-center">
      {/* HEADER */}
      <header className="w-full max-w-6xl flex justify-between items-center mb-6 border-b border-slate-800 pb-4">
        <div>
          <h1 className="text-2xl md:text-4xl font-black italic tracking-tighter text-transparent bg-clip-text bg-gradient-to-r from-yellow-400 to-orange-600">
            BEAST INDEX ARENA
          </h1>
          <div className="flex gap-2 mt-2">
             <span className="px-2 py-0.5 bg-blue-500/10 text-blue-400 text-[10px] rounded border border-blue-500/20">SOLANA DEVNET</span>
             <span className="px-2 py-0.5 bg-green-500/10 text-green-400 text-[10px] rounded border border-green-500/20 flex items-center gap-1">
                <Coins size={10} /> BALANCE: ${balance}
             </span>
          </div>
        </div>
        <div className="text-right">
           <div className="text-xs text-slate-500 font-bold tracking-widest">TURN {turn}</div>
           <div className="text-xl font-bold text-white animate-pulse">LIVE</div>
        </div>
      </header>

      {/* ARENA GRID */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 w-full max-w-6xl mb-6">
        {creatures.map((beast) => (
          <div 
            key={beast.id} 
            className={`relative p-4 rounded-xl border-2 transition-all duration-300 flex flex-col ${beast.alive ? `${beast.color} ${beast.borderColor}` : "bg-slate-900 border-slate-800 opacity-50 grayscale"}`}
          >
            {/* Header: Flag & Name */}
            <div className="flex justify-between items-start mb-2">
                <span className="text-2xl">{beast.country}</span>
                {/* NEW: VISIBLE HP BIG TEXT */}
                <div className="text-right">
                    <span className={`text-2xl font-black ${beast.hp < 30 ? "text-red-500 animate-pulse" : "text-white"}`}>
                        {beast.hp}
                    </span>
                    <span className="text-xs text-slate-400 block">/ {beast.maxHp} HP</span>
                </div>
            </div>

            {/* Sprite */}
            <div className="h-32 w-full mb-4 flex items-center justify-center text-7xl drop-shadow-2xl">
              <span className={beast.alive ? "animate-bounce-slow" : "blur-sm"}>
                {beast.sprite}
              </span>
            </div>
            
            <h2 className="text-center font-black text-xl mb-4 tracking-tight">{beast.name}</h2>

            {/* HP Bar Visual */}
            <div className="w-full bg-black/40 h-2 rounded-full overflow-hidden mb-4 border border-white/5">
                <motion.div 
                  initial={{ width: "100%" }}
                  animate={{ width: `${(beast.hp / beast.maxHp) * 100}%` }}
                  className={`h-full ${beast.hp < 30 ? "bg-red-500" : "bg-green-400"}`}
                />
            </div>

            {/* NEW: TRADING INTERFACE */}
            <div className="mt-auto bg-black/30 p-3 rounded-lg border border-white/5">
                <div className="flex justify-between text-xs text-slate-400 mb-2">
                    <span>POS: <strong>{beast.userShares} SHARES</strong></span>
                    <span>PNL: <span className="text-green-400">+0%</span></span>
                </div>
                
                <div className="grid grid-cols-2 gap-2">
                    {/* BUY BUTTON */}
                    <button 
                        onClick={() => handleTrade(beast.id, 'buy')}
                        disabled={!beast.alive}
                        className="flex items-center justify-center gap-1 bg-green-600/20 hover:bg-green-600 text-green-400 hover:text-white py-2 rounded font-bold text-xs border border-green-600/50 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        <TrendingUp size={14} /> BUY
                    </button>

                    {/* SELL BUTTON */}
                    <button 
                        onClick={() => handleTrade(beast.id, 'sell')}
                        disabled={!beast.alive || beast.userShares <= 0}
                        className="flex items-center justify-center gap-1 bg-red-600/20 hover:bg-red-600 text-red-400 hover:text-white py-2 rounded font-bold text-xs border border-red-600/50 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        <TrendingDown size={14} /> SELL
                    </button>
                </div>
            </div>

            {!beast.alive && (
                <div className="absolute inset-0 flex items-center justify-center bg-black/60 backdrop-blur-sm rounded-xl z-10">
                    <div className="text-center">
                        <ShieldAlert className="w-12 h-12 text-red-600 mx-auto mb-2" />
                        <span className="text-red-500 font-black text-xl border-2 border-red-600 px-4 py-1 rounded uppercase rotate-12 inline-block">Eliminated</span>
                    </div>
                </div>
            )}
          </div>
        ))}
      </div>

      {/* LOGS CONSOLE */}
      <div className="w-full max-w-6xl bg-black rounded-xl border border-slate-800 p-4 font-mono text-xs md:text-sm h-64 flex flex-col shadow-2xl">
        <div className="flex items-center gap-2 mb-2 border-b border-slate-800 pb-2">
            <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
            <span className="text-slate-400 font-bold tracking-widest">LIVE COMBAT LOG</span>
        </div>
        <div className="flex-1 overflow-y-auto space-y-1 pr-2 scrollbar-thin scrollbar-thumb-slate-700">
          <AnimatePresence mode="popLayout">
            {logs.map((log, i) => (
              <motion.div 
                key={`${turn}-${i}`}
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                className={`pl-2 border-l-2 py-0.5 ${log.includes("ELIMINATED") ? "text-red-400 border-red-500 font-bold bg-red-900/10" : "text-slate-300 border-slate-700"}`}
              >
                <span className="text-slate-600 mr-2 text-[10px]">{new Date().toLocaleTimeString()}</span>
                {log}
              </motion.div>
            ))}
            <div ref={logsEndRef} />
          </AnimatePresence>
          {logs.length === 0 && <span className="text-slate-700 italic">Initializing battle sequence...</span>}
        </div>
      </div>
    </main>
  );
}