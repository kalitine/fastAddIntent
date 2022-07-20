use std::vec;

type ID = u32;
type Relation = Vec<(ID, Vec<ID>)>;
type Lattice = Vec<Concept>;

struct Concept {
	extent: Vec<ID>,
	intent: Vec<ID>,
	parents: Vec<&Concept>,
}

impl Concept {
	fn new(extent: Vec<ID>, intent: Vec<ID>) -> Concept {
		Concept {
			extent,
			intent,
			parents: Vec::new(),
		}
	}

	fn add_object_all_up(&mut self, object: ID) {
		if self.extent.iter().all(|&i| i != object) {
			self.extent.push(object);
		}
		for p in self.parents.iter_mut() {
			p.add_object_all_up(object);
		}
	}

	fn clone(&self) -> Concept {
		let copy = Concept::new(self.extent.clone(), self.intent.clone());
		copy.parents = self.parents.clone();
		return copy;
	}
}

fn main() {
	let relation: Relation = vec![
		(1, vec![4, 5, 6]),
		(2, vec![4, 5, 6]),
		(3, vec![2, 3]),
		(4, vec![2, 3]),
	];
	let extent = vec![1, 2, 3, 4];
	let intent = vec![1, 2, 3, 4, 5, 6];
	create_lattice_incrementally(&extent, &intent, &relation);
}

/**
 * Algorithm 1. Procedure GetClosureConcept(Y, generator, L):
 *
 */
fn get_closure() {
	// 1: if L.Find(Y) != £ then:
	// 2: return L.Find(Y)
	// 3: end if
	// 4: closureExtent = £
	// 5: for each attribute in Y:
	// 6: if closureExtent = = £ then:
	// 7: closureExtent = {attribute}
	// ;I
	// 8: else:
	// 9: closureExtent = closureExtent \ {attribute}
	// ;I
	// 10: end if
	// 11: end for
	// 12: closureIntent = Y
	// 13: for each m in generator.intent-Y:
	// 14: if closureExtent # {m}
	// ;I
	// then:
	// 15: closureIntent = closureIntent [ {m}
	// 16: end if
	// 17: end for
	// 18: return L.Find(closureIntent)
}

/**
 * Algorithm 2: Procedure FastAddIntent(Y, generator, L)
 */
fn fast_add_intent(intent: &Vec<ID>, generator: &Concept, l: &mut Lattice) -> Concept {
	// 1: generator = GetClosureConcept(Y, generator, L)
	// 2: if generator.Intent == Y then:
	// 3: return generator
	// 4: end if
	// 5: GeneratorParents = generator.Parents
	// 6: newParents = £
	// 7: for each candidate in GeneratorParents:
	// 8: if candidate.Intent å Y then:
	// 9: candidate = FastAddIntent(candidate.Intent \
	// Y,candidate,L)
	// 10: end if
	// 11: Add candidate to newParents
	// 12: end for
	// 13: newConcept = (generator.Extent,Y)
	// 14: L = L [ {newConcept}
	// 15: for each parent in newParents:
	// 16: isTrueParent = True
	// 17: chidren = parent.Children
	// 18: for each child in children:
	// 19: if child.Intent # Y then:
	// 20: isTrueParent = False
	// 21: break
	// 22: end if
	// 23: end for
	// 24: if isTrueParent then:
	// 25: Remove links between generator and parent
	// 26: Set links between newConcept and parent
	// 27: end if
	// 28: end for
	// 29: Set links between generator and newConcept
	// 30: return newConcept
	return Concept::new(Vec::new(), Vec::new());
}

/**
 * Algorithm 3: Procedure CreateLatticeIncrementally(G, M, I)
 * G = extent
 * M = intent
 * I = relation
 */
fn create_lattice_incrementally(
	extent: &Vec<ID>,
	intent: &Vec<ID>,
	relation: &Relation,
) -> Lattice {
	// 1: bottomConcept = (£, M)
	let bottom_concept = Concept::new(Vec::new(), intent.clone());
	// 2: L = {bottomConcept}
	let mut lattice = vec![bottom_concept];
	// 3: for each g in G:
	for o in *extent {
		println!("Extent #{} | Intent ", o);
		// 4: objectConcept = FastAddIntent(g"I’, bottomConcept, L)
		let mut object_concept =
			fast_add_intent(get_intent(o, &relation), lattice[0].clone(), &mut lattice);
		// 5: Add g to the extent of objectConcept and all concepts above
		object_concept.add_object_all_up(o);
		// 6: end for
	}
	// 7: return L
	return lattice;
}

fn get_intent(object: ID, relation: &Relation) -> &Vec<ID> {
	for (o, intent) in relation {
		if *o == object {
			return intent;
		}
	}
	panic!("Object {} was not found.", object);
}
