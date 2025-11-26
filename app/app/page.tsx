"use client";

import { useState, useEffect, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";

// --- TYPES ---
type Creature = {
  id: number;
  name: string;
  country: string; // Emoji flag
  sprite: string;  // Placeholder Emoji
  hp: number;
  maxHp: number;
  alive: boolean;
  shares: number;
  color: string;
  borderColor: string;
};

// --- CONFIGURATION: THE FINAL SQUAD ---
const INITIAL_STATE: Creature[] = [
  { 
    id: 0, 
    name: "YETI", 
    country: "🇳🇵", 
    sprite: "❄️", 
    hp: 100, maxHp: 100, alive: true, shares: 1.0, 
    color: "bg-blue-900/40", borderColor: "border-blue-400" 
  },
  { 
    id: 1, 
    name: "MAPINGUARI", 
    country: "🇧🇷", 
    sprite: "🌿", 
    hp: 100, maxHp: 100, alive: true, shares: 1.0, 
    color: "bg-amber-900/40", borderColor: "border-amber-600" 
  },
  { 
    id: 2, 
    name: "ZMEY", 
    country: "🇷🇺", 
    sprite: "🔥", // Fire dragon
    hp: 100, maxHp: 100, alive: true, shares: 1.0, 
    color: "bg-red-900/40", borderColor: "border-red-500" 
  },
  { 
    id: 3, 
    name: "NAGA", 
    country: "🇮🇳", 
    sprite: "🐍", 
    hp: 100, maxHp: 100, alive: true, shares: 1.0, 
    color: "bg-emerald-900/40", borderColor: "border-emerald-400" 
  },
];

export default function Arena() {
  const [creatures, setCreatures] = useState<Creature[]>(INITIAL_STATE);
  const [logs, setLogs] = useState<string[]>([]);
  const [turn, setTurn] = useState(0);
  const logsEndRef = useRef<HTMLDivElement>(null);

  // --- MOCK CRANK (Simulation) ---
  useEffect(() => {
    const interval = setInterval(() => {
      executeMockTurn();
    }, 2000); // 2 seconds per turn (Fast)
    return () => clearInterval(interval);
  }, [creatures]);

  // Auto-scroll logs
  useEffect(() => {
    logsEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [logs]);

  const executeMockTurn = () => {
    const alive = creatures.filter((c) => c.alive);
    if (alive.length <= 1) return;

    // RNG Logic
    const attacker = alive[Math.floor(Math.random() * alive.length)];
    const targets = alive.filter((c) => c.id !== attacker.id);
    const target = targets[Math.floor(Math.random() * targets.length)];
    const damage = Math.floor(Math.random() * 12) + 5; // Damage 5-17

    // Update HP
    const updated = creatures.map((c) => {
      if (c.id === target.id) {
        const newHp = Math.max(0, c.hp - damage);
        return { ...c, hp: newHp, alive: newHp > 0 };
      }
      return c;
    });

    setCreatures(updated);
    setTurn((prev) => prev + 1);
    
    // Add Log
    const newLog = `Turn ${turn + 1}: ${attacker.name} hits ${target.name} (-${damage})`;
    setLogs((prev) => [...prev, newLog]); // Append log
  };

  return (
    <main className="min-h-screen bg-slate-950 text-white p-2 md:p-8 font-mono flex flex-col items-center">
      {/* HEADER */}
      <header className="w-full max-w-5xl flex justify-between items-center mb-6 border-b border-slate-800 pb-4">
        <div>
          <h1 className="text-2xl md:text-4xl font-black text-transparent bg-clip-text bg-gradient-to-r from-yellow-400 to-orange-500 italic tracking-tighter">
            BEAST INDEX ARENA
          </h1>
          <div className="flex gap-2 mt-1">
             <span className="px-2 py-0.5 bg-blue-500/20 text-blue-400 text-[10px] rounded border border-blue-500/50">SOLANA DEVNET</span>
             <span className="px-2 py-0.5 bg-purple-500/20 text-purple-400 text-[10px] rounded border border-purple-500/50">PARIMUTUEL</span>
          </div>
        </div>
        <div className="text-right">
           <div className="text-xs text-slate-400">NEXT TURN IN</div>
           <div className="text-xl font-bold text-white animate-pulse">00:02</div>
        </div>
      </header>

      {/* ARENA GRID */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-3 w-full max-w-5xl mb-6">
        {creatures.map((beast) => (
          <div 
            key={beast.id} 
            className={`relative p-3 rounded-xl border-2 transition-all duration-300 flex flex-col ${beast.alive ? `${beast.color} ${beast.borderColor}` : "bg-slate-900 border-slate-800 opacity-40 grayscale"}`}
          >
            {/* Country Flag Badge */}
            <div className="absolute top-2 left-2 bg-black/30 px-2 py-1 rounded text-sm">
                {beast.country}
            </div>

            {/* Sprite Area */}
            <div className="h-28 md:h-40 w-full mb-2 flex items-center justify-center text-7xl drop-shadow-2xl">
              <span className={beast.alive ? "animate-bounce" : "blur-sm"}>
                {beast.sprite}
              </span>
            </div>

            {/* Name & HP */}
            <div className="mt-auto bg-black/20 p-2 rounded">
              <h2 className="text-lg font-black leading-none mb-1 text-center">{beast.name}</h2>
              
              {/* HP Bar */}
              <div className="w-full bg-slate-900 h-3 rounded-full overflow-hidden border border-white/10 relative">
                <motion.div 
                  initial={{ width: "100%" }}
                  animate={{ width: `${(beast.hp / beast.maxHp) * 100}%` }}
                  className={`h-full absolute top-0 left-0 ${beast.hp < 30 ? "bg-red-500" : "bg-green-400"}`}
                />
                <div className="absolute inset-0 flex items-center justify-center text-[9px] font-bold z-10 text-white drop-shadow-md">
                    {beast.hp}/{beast.maxHp} HP
                </div>
              </div>
            </div>

            {/* Buy Button */}
            <button 
              disabled={!beast.alive}
              className="w-full mt-2 bg-white text-black py-2 rounded text-xs font-black uppercase hover:bg-yellow-400 disabled:bg-slate-800 disabled:text-slate-600 transition-colors shadow-lg"
            >
              {beast.alive ? "BUY SHARE" : "ELIMINATED"}
            </button>
          </div>
        ))}
      </div>

      {/* LOGS CONSOLE */}
      <div className="w-full max-w-5xl bg-black rounded-lg border border-slate-800 p-4 font-mono text-xs md:text-sm h-48 overflow-hidden shadow-2xl flex flex-col">
        <div className="flex items-center gap-2 mb-2 border-b border-slate-800 pb-2">
            <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
            <span className="text-slate-400 font-bold tracking-widest">LIVE COMBAT LOG</span>
        </div>
        <div className="flex flex-col gap-1.5 overflow-y-auto pr-2">
          <AnimatePresence>
            {logs.map((log, i) => (
              <motion.div 
                key={`${turn}-${i}`}
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                className="text-slate-300 border-l-2 border-slate-700 pl-2"
              >
                <span className="text-slate-600 mr-2">{`>`}</span>
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