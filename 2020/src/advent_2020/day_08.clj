(ns advent-2020.day-08
  (:require [clojure.java.io :as io]
            [clojure.string :as string]))

(defn process-input [i]
  (->> i
       (mapv (fn [s] (string/split s #" ")))
       (mapv (fn [[s n]] [s (Long/parseLong n)]))))

(def input
  (-> "advent_2020/day_08/input"
      io/resource
      slurp
      string/split-lines
      process-input))

(defn step [coll ps p accum]
  (let [[s n] (nth coll p)
                p-ps (conj ps p)]
            (case s
              "nop" [p-ps (inc p) accum]
              "acc" [p-ps (inc p) (+ accum n)]
              "jmp" [p-ps (+ p n) accum])))

(def execute-input (loop [[ps p accum] [#{} 0 0]]
                     (if (ps p)
                       [ps accum]
                       (recur (step input ps p accum)))))

(def part-1 (second execute-input))

(defn accumulate
  [coll]
  (loop [[ps p accum] [#{} 0 0]]
    (cond (= p (count input))
          accum
          (ps p)
          nil
          :else
          (recur (step coll ps p accum)))))

(def part-2
  (->> (map (fn [n]
              (case (get-in input [n 0])
                "nop" (assoc-in input [n 0] "jmp")
                "jmp" (assoc-in input [n 0] "nop")
                "acc" nil)) (first execute-input))
       (filter some?)
       (map accumulate)
       (filter some?)
       first))

