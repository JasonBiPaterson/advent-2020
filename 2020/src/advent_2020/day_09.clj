(ns advent-2020.day-09
  (:require [clojure.java.io :as io]
            [clojure.string :as string]))

(def input
  (->> "advent_2020/day_09/input"
       io/resource
       slurp
       string/split-lines
       (map #(Long/parseLong %))))

(defn match? [n coll]
  (some (fn [v]
          (and (not= n (* 2 v))
               (some #{(- n v)} coll)))
        coll))

(def part-1
  (let [preamble 25]
    (->> (drop preamble input)
         (map-indexed
          (fn [idx v]
            (when-not (match? v (->> input
                                     (take (+ preamble idx))
                                     (drop idx)))
              v)))
         (some identity))))

(def part-2
  (let [coll (take-while #(not= part-1 %) input)
        lol (butlast (map-indexed (fn [idx _] (drop idx coll)) coll))
        [[begin end]] (for [l lol
                            :let [end (reduce (fn [a b]
                                                (let [ab (+ a b)]
                                                  (cond (= part-1 ab)
                                                        (reduced b)
                                                        (< part-1 ab)
                                                        (reduced nil)
                                                        :else
                                                        ab)))
                                              l)]
                            :when end]
                        [(first l) end])
        contiguous (->> input
                        (drop-while #(not= begin %))
                        (take-while #(not= end %))
                        (concat [end]))]
    (+ (apply min contiguous) (apply max contiguous))))
