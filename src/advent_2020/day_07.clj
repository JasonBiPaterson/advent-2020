(ns advent-2020.day-07
  (:require [clojure.java.io :as io]
            [medley.core :as medley]
            [clojure.string :as string]
            [clojure.set :as set]))
medley/map-vals

(defn process-input [i]
  (into {}
        (for [line i
              :let [[k v] (string/split line #" bags contain ")]]
          [k
           (if (= "no other bags." v)
             {}
             (->> (string/split v #", ")
                  (mapv (fn [s]
                          (-> s
                              (string/replace #"(bags|bag|\.)" "")
                              (string/split #" " 2)
                              reverse
                              vec
                              (update 0 string/trim)
                              (update 1 #(Long/parseLong %)))))
                  (into {})))])))

(def input
  (-> "advent_2020/day_07/input"
      io/resource
      slurp
      string/split-lines
      process-input))

(defn contains-bag [bag]
  (for [[k v] input
        :when (contains? v bag)]
    k))

(def part-1
  (count
   (loop [bags ["shiny gold"]
          ans-bags []]
     (let [found-bags (distinct
                       (mapcat contains-bag bags))]
       (if (empty? found-bags)
         ans-bags
         (recur found-bags (-> ans-bags
                               (concat found-bags)
                               distinct)))))))

(def part-2
  (->> (loop [bag-counts {"shiny gold" 1}
              results {}]
         (let [child-bag-counts (apply merge-with +
                                       (for [[bag n] bag-counts]
                                         (->> (get input bag)
                                              (medley/map-vals #(* % n)))))]
           (if (empty? bag-counts)
             results
             (recur child-bag-counts (merge-with + results bag-counts)))))
       vals
       (reduce +)
       dec))
