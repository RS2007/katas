import java.util.LinkedList
import java.util.Queue
import java.util.Arrays

class Node(val value: Int,val _left: Option[Node],val _right: Option[Node]){
  override def toString() = "Node: value = " + value.toString() + "\t left = " + _left.toString() + "\t right = " + _right.toString()
  def toBfsIter =  BfsIterator((this,0))
  def zigzagLevel =
  this
  .toBfsIter
  .toList // collect from iterator
  .groupBy(_._2) // create a hashmap with the second element of tuple as key
  .toSeq
  .sortBy(_._1) // sort based on hashmap keys
  .map { // reversing based on parity
     case(key,value) =>{
         if key % 2 == 1 then value.reverse else value
     }
  }
  .map(_.map(_._1.value)) // flatten to get the values in the nodes
  .toList

}


class BfsIterator(node: (Node,Int)) extends Iterator[(Node,Int)] {
  val bfsCache: Queue[(Node,Int)] = new LinkedList(Arrays.asList(node))
  var level = 0
  var hasNext = true
  def next() = {
    val currNode = this.bfsCache.poll()
    currNode(0)._left match {
      case Some(k) => this.bfsCache.add((k, this.level+1))
      case _ => 
    }
    currNode(0)._right match {
      case Some(k) => this.bfsCache.add((k,this.level+1))
      case _ => 
    }

    if this.bfsCache.size() == 0 then
      this.hasNext = false
    this.level = this.level + 1
    return currNode 
  }
}



@main def main(): Unit =
  var d = Node(4, None, None)
  var b = Node(1, None, None)
  var a = Node(2, None, Some(d))
  var k = Node(3, Some(a), Some(b))
  println(k.zigzagLevel)

